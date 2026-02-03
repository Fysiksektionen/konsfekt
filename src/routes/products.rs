use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::{self, Data}};
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use sqlx::SqlitePool;

use crate::{AppError, AppState, Role, database::{self, model::User}, model::{PendingTransaction, Product, ProductParams}, routes::user_from_cookie, utils};

fn product_assert_permission(product: &Product, user: &User) -> Result<(), AppError> {
    // Check if product may be modified
    if !product.flags.modifiable && user.role != Role::Admin {
        return Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Product not modifiable")));
    }

    Ok(())
}

async fn get_product_from_id(pool: &SqlitePool, id: Option<u32>) -> Result<Product, AppError> {
    let id = id.ok_or(AppError::BadRequest("Missing required argument \"id\"".to_string()))?;

    let product_row = database::crud::get_product(pool, id).await?;
    let product = Product::from_row(product_row)
        .map_err(|_| AppError::GenericError("Internal Database formatting incorrect".to_string()))?;
    
    Ok(product)
}

#[derive(MultipartForm)]
struct ProductAndImageForm {
    #[multipart(limit = "100MB")]
    image: Option<TempFile>,
    product: MpJson<ProductParams>,
}

#[post("/api/create_product")]
pub async fn create_product(state: Data<AppState>, MultipartForm(form): MultipartForm<ProductAndImageForm>) -> Result<impl Responder, AppError> {
    let product = Product::from_request(form.product.into_inner())
        .map_err(|_| AppError::BadRequest("Missing required arguments".to_string()))?;
    let product_row = database::crud::create_product(&state.db, product.into_row()).await?;
    
    if let Some(file) = form.image {
        if utils::save_img_to_disk(file, &product_row.id.to_string()).is_none() {
            return Err(AppError::GenericError("Product image not saved".to_string())) 
        }
    }
    let products = database::crud::get_products(&state.db).await?;

    Ok(HttpResponse::Ok().json(products))
}


#[post("/api/update_product")]
pub async fn update_product(state: Data<AppState>, req: HttpRequest, MultipartForm(form): MultipartForm<ProductAndImageForm>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    let mut product = get_product_from_id(&state.db, form.product.id).await?;
    let params = form.product.into_inner();

    product_assert_permission(&product, &user)?;
    
    product.update(params);

    // Remove marked as sold if restocked
    if product.stock.is_some_and(|s| s > 0) || product.stock.is_none() {
        product.flags.marked_sold_out = false; 
    }
    
    database::crud::update_product_data(&state.db, product.clone().into_row()).await?;

    if let Some(file) = form.image {
        if utils::save_img_to_disk(file, &product.id.to_string()).is_none() {
            return Err(AppError::GenericError("Product image not saved".to_string())) 
        }
    }

    let products = database::crud::get_products(&state.db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[post("/api/mark_sold_out")]
pub async fn mark_sold_out(state: Data<AppState>, params: web::Json<ProductIdJson>) -> Result<impl Responder, AppError> {
    let mut product = get_product_from_id(&state.db, Some(params.id)).await?;

    if product.stock.is_none() {
        return Err(AppError::ActixError(actix_web::error::ErrorConflict("Cannot mark product not for sale as sold out")));
    }
    
    product.flags.marked_sold_out = true;

    database::crud::update_product_data(&state.db, product.clone().into_row()).await?;

    Ok(HttpResponse::Ok())
}

#[derive(serde::Deserialize)]
struct ProductIdJson { id: u32 }

#[post("/api/delete_product")]
pub async fn delete_product(state: Data<AppState>, req: HttpRequest, params: web::Json<ProductIdJson>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    let product = get_product_from_id(&state.db, Some(params.id)).await?;

    product_assert_permission(&product, &user)?;
    database::crud::delete_product(&state.db, product.id).await?;

    let products = database::crud::get_products(&state.db).await?;

    let _ = utils::delete_img_from_disk(&format!("{}", product.id));

    Ok(HttpResponse::Ok().json(products))
}

#[get("/api/get_products")]
pub async fn get_products(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let products = database::crud::get_products(&state.db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[derive(serde::Deserialize)]
struct Cart { 
    products: Vec<ProductInCart>
}

#[derive(serde::Deserialize)]
struct ProductInCart {
    id: u32,
    quantity: u32,
}

#[post("/api/buy_products")]
pub async fn buy_products(state: Data<AppState>, req: HttpRequest, cart: web::Json<Cart>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    let mut products = Vec::new();
    for p in &cart.products {
        let product = database::crud::get_product(&state.db, p.id).await?;
        // Products can be out of stock in database but exist in godisskåp
        if product.stock.is_none() {
            return Err(AppError::ActixError(actix_web::error::ErrorNotFound("Product not available")))
        }
        products.push((product, p.quantity));
    }
    let total_price = products.iter().fold(0.0, |tot, (p, quantity)| tot + p.price * (*quantity as f32));

    if total_price > user.balance {
        return Err(AppError::ActixError(actix_web::error::ErrorPaymentRequired("Not enough funds")));
    }

    let transaction = PendingTransaction {
        user: user.id,
        products: products.clone(),
        amount: -total_price
    };
     
    database::crud::create_transaction(&state.db, transaction).await?;

    database::crud::update_user_balance(&state.db, user.id, user.balance - total_price).await?;
    
    for (product, quantity) in products {
        let new_stock = Some(product.stock.unwrap() - quantity as i32);
        database::crud::update_product_stock(&state.db, product.id, new_stock).await?;
    }

    Ok(HttpResponse::Ok())
}
