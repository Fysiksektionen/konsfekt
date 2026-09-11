use actix_web::{get, post, web::{self, Data, Json}};
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::SqlitePool;
use time::OffsetDateTime;

type HmacSha256 = Hmac<Sha256>;

use crate::{AppState, Role, database::{self, crud, model::UserRow}, error::{ApiResult, AppError, GenericError}, model::{PendingTransaction, Product, ProductParams, TransactionDetail}, return_err, routes::CurrentUser, utils};

fn product_assert_permission(product: &Product, user: &UserRow) -> ApiResult<()> {
    if !product.flags.modifiable && user.role != Role::Admin {
        return_err!(actix_web::error::ErrorForbidden("Product not modifiable"));
    }
    Ok(())
}

async fn get_product_from_id(pool: &SqlitePool, id: Option<u32>) -> ApiResult<Product> {
    let Some(id) = id else {
        return_err!(actix_web::error::ErrorBadRequest("Missing required argument \"id\""));
    };
    let product_row = database::crud::get_product(pool, id).await?;
    let product = Product::from_row(product_row)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Internal database formatting incorrect"))?;
    Ok(product)
}

#[derive(MultipartForm)]
struct ProductAndImageForm {
    #[multipart(limit = "100MB")]
    image: Option<TempFile>,
    product: MpJson<ProductParams>,
}

#[derive(serde::Deserialize)]
struct ProductIdJson { id: u32 }

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct PurchaseResponse {
    transaction_id: u32,
    token: Option<String>, // Only needed if user has anonymous transactions enabled
}

fn create_undo_token(secret: &str, transaction_id: u32, user_id: u32) -> Result<String, AppError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| GenericError::new("Could not initialize HMAC"))?;
    // Bind the transaction and user to the token
    mac.update(&transaction_id.to_le_bytes());
    mac.update(&user_id.to_le_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn verify_undo_token(secret: &str, transaction_id: u32, user_id: u32, token_hex: &str) -> Result<bool, AppError> {
    let Ok(token) = hex::decode(token_hex) else { return Ok(false) };
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| GenericError::new("Could not initialize HMAC"))?;
    mac.update(&transaction_id.to_le_bytes());
    mac.update(&user_id.to_le_bytes());
    Ok(mac.verify_slice(&token).is_ok())
}

#[post("/api/create_product")]
pub async fn create_product(state: Data<AppState>, current_user: CurrentUser, MultipartForm(form): MultipartForm<ProductAndImageForm>) -> ApiResult<impl actix_web::Responder> {
    current_user.require_role(Role::Maintainer)?;

    let product = Product::from_request(form.product.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Missing required arguments"))?;
    let product_row = database::crud::create_product(&state.db, product.into_row()).await?;

    if let Some(file) = form.image {
        if utils::save_img_to_disk(file, &product_row.id.to_string()).is_none() {
            return_err!(actix_web::error::ErrorInternalServerError("Product image not saved"));
        }
    }
    let products = database::crud::get_products(&state.db).await?;

    Ok(web::Json(products))
}

#[post("/api/update_product")]
pub async fn update_product(state: Data<AppState>, current_user: CurrentUser, MultipartForm(form): MultipartForm<ProductAndImageForm>) -> ApiResult<impl actix_web::Responder> {
    // let user = user_from_cookie(&state.db, &req).await?;
    let user = current_user.require_role(Role::Maintainer)?.into_row();
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
            return_err!(actix_web::error::ErrorInternalServerError("Product image not saved"));
        }
    }

    let products = database::crud::get_products(&state.db).await?;

    Ok(web::Json(products))
}

#[post("/api/mark_sold_out")]
pub async fn mark_sold_out(state: Data<AppState>, params: web::Json<ProductIdJson>) -> ApiResult<()> {
    let mut product = get_product_from_id(&state.db, Some(params.id)).await?;

    if product.stock.is_none() {
        return_err!(actix_web::error::ErrorConflict("Cannot mark product not for sale as sold out"));
    }

    product.flags.marked_sold_out = true;
    database::crud::update_product_data(&state.db, product.clone().into_row()).await?;

    Ok(())
}

#[post("/api/delete_product")]
pub async fn delete_product(state: Data<AppState>, current_user: CurrentUser, params: web::Json<ProductIdJson>) -> ApiResult<impl actix_web::Responder> {
    // let user = user_from_cookie(&state.db, &req).await?;
    let user = current_user.require_role(Role::Maintainer)?.into_row();
    let product = get_product_from_id(&state.db, Some(params.id)).await?;

    product_assert_permission(&product, &user)?;
    database::crud::delete_product(&state.db, product.id).await?;

    let products = database::crud::get_products(&state.db).await?;
    let _ = utils::delete_img_from_disk(&format!("{}", product.id));

    Ok(web::Json(products))
}

#[get("/api/get_products")]
pub async fn get_products(state: Data<AppState>) -> ApiResult<impl actix_web::Responder> {
    let products = database::crud::get_products(&state.db).await?;
    Ok(web::Json(products))
}

#[post("/api/buy_single_product")]
pub async fn buy_single_product(state: Data<AppState>, current_user: CurrentUser, product: web::Json<ProductIdJson>) -> ApiResult<Json<PurchaseResponse>> {
    // let user = user_from_cookie(&state.db, &req).await?;
    let user = current_user.require_role(Role::User)?.into_row();
    let product = database::crud::get_product(&state.db, product.id).await?;

    if product.stock.is_none() {
        return_err!(actix_web::error::ErrorNotFound("Product not available"));
    }
    if product.price > user.balance {
        return_err!(actix_web::error::ErrorPaymentRequired("Not enough funds"));
    }

    let transaction = PendingTransaction {
        user: match user.private_transactions {
            true => None,
            false => Some(user.id)
        },
        products: vec![(product.clone(), 1)],
        amount: -product.price,
        admin_issued: false
    };

    let transaction_id = database::crud::create_transaction(&state.db, transaction).await?;
    database::crud::update_user_balance(&state.db, user.id, user.balance - product.price).await?;

    let new_stock = Some(product.stock.unwrap() - 1 as i32);
    database::crud::update_product_stock(&state.db, product.id, new_stock).await?;

    let token = create_undo_token(&state.env.undo_purchase_secret, transaction_id, user.id)?;
    Ok(Json(PurchaseResponse { transaction_id, token: Some(token) }))
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
pub async fn buy_products(state: Data<AppState>, current_user: CurrentUser, cart: web::Json<Cart>) -> ApiResult<Json<PurchaseResponse>> {
    let user = current_user.require_role(Role::User)?.into_row();
    let mut products = Vec::new();
    for p in &cart.products {
        let product = database::crud::get_product(&state.db, p.id).await?;
        if product.stock.is_none() {
            return_err!(actix_web::error::ErrorNotFound("Product not available"));
        }
        products.push((product, p.quantity));
    }
    let total_price = products.iter().fold(0.0, |tot, (p, quantity)| tot + p.price * (*quantity as f32));

    if total_price > user.balance {
        return_err!(actix_web::error::ErrorPaymentRequired("Not enough funds"));
    }

    let transaction = PendingTransaction {
        user: match user.private_transactions {
            true => None,
            false => Some(user.id)
        },
        products: products.clone(),
        amount: -total_price,
        admin_issued: false
    };

    let transaction_id = database::crud::create_transaction(&state.db, transaction).await?;
    database::crud::update_user_balance(&state.db, user.id, user.balance - total_price).await?;

    for (product, quantity) in products {
        // Unwrap will never panic because product.stock.is_none check above
        let new_stock = Some(product.stock.unwrap() - quantity as i32);
        database::crud::update_product_stock(&state.db, product.id, new_stock).await?;
    }

    let token = create_undo_token(&state.env.undo_purchase_secret, transaction_id, user.id)?;
    Ok(Json(PurchaseResponse { transaction_id, token: Some(token) }))
}

#[post("/api/undo_purchase")]
pub async fn undo_purchase(state: Data<AppState>, current_user: CurrentUser, purchase_response: web::Json<PurchaseResponse>) -> ApiResult<()> {
    // let user = user_from_cookie(&state.db, &req).await?;
    let user = current_user.require_role(Role::User)?.into_row();
    let transaction = database::crud::get_transaction(&state.db, purchase_response.transaction_id).await?;

    if transaction.amount > 0.0 {
        return_err!(actix_web::error::ErrorConflict("Cannot undo a deposit"));
    }

    match transaction.user {
        Some(owner) if owner == user.id => {}
        Some(_) => {
            return_err!(actix_web::error::ErrorForbidden("Cannot undo another user's transaction"));
        }
        None => {
            let Some(token) = &purchase_response.token else {
                return_err!(actix_web::error::ErrorForbidden("Need purchase token to undo purchase"));
            };
            let is_valid_token = verify_undo_token(
                &state.env.undo_purchase_secret, 
                purchase_response.transaction_id, 
                user.id,
                &token)?;
            if !is_valid_token {
                return_err!(actix_web::error::ErrorForbidden("Could not verify ownership of purchase"));
            }
        }
    }
    if OffsetDateTime::now_utc().unix_timestamp() - transaction.datetime > 60 {
        return_err!(actix_web::error::ErrorConflict("Transaction cannot be undone anymore"));
    }

    let user_id = user.id;
    let full_transaction = crud::get_detailed_transaction(&state.db, purchase_response.transaction_id, user).await?;

    database::crud::undo_purchase(&state.db, purchase_response.transaction_id, user_id, transaction.amount).await?;
    
    for item in full_transaction.items {
        let product = database::crud::get_product(&state.db, item.product_id).await?;
        if let Some(stock) = product.stock {
            // Only update stock currently for sale
            let new_stock = Some(stock + item.quantity as i32);
            database::crud::update_product_stock(&state.db, product.id, new_stock).await?;
        }
    }

    Ok(())
}
