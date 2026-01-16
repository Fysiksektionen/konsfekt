use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, body::BoxBody, cookie::Cookie, dev::{ServiceRequest, ServiceResponse}, get, middleware, post, web::{self, Data}};
use serde::{Deserialize, Serialize};
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use sqlx::SqlitePool;
use time::Duration;

use crate::{AppError, AppState, Role, auth, database::{self, crud, model::User}, model::{Product, ProductParams, Transaction}, utils::{self, get_path}};

const LOGIN_PATH: &str = "/login";
const PATH_WHITELIST: [&str; 3] = [
    LOGIN_PATH,
    "/api/auth/google",
    "/api/auth/google/callback",
];

//
//          Middleware
//

/// Redirects to `path` taking into account where the frontend is served
fn redirect_response(state: Data<AppState>, req: ServiceRequest, path: &str) -> ServiceResponse {
    let response = HttpResponse::Found()
        .append_header(("Location", utils::get_path(&state, path)))
        .finish();
    req.into_response(response)
} 

pub async fn session_middleware(
    state: Data<AppState>,
    req: ServiceRequest, 
    next: middleware::Next<BoxBody>
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let path = req.path();
    
    if PATH_WHITELIST.contains(&path) {
        return next.call(req).await;
    }
    
    match auth::parse_auth_cookie(req.cookie(auth::AUTH_COOKIE)) {

        // Cookie not found
        None => if path != get_path(&state, LOGIN_PATH) { Ok(redirect_response(state, req, LOGIN_PATH)) }
                else { next.call(req).await }
        
        Some(token) => {
            match auth::validate_session(&state.db, token).await {

                // Validation Good
                Ok(Some(session)) => {
                    req.extensions_mut().insert(session);
                    if path == LOGIN_PATH { return Ok(redirect_response(state, req, "/")) };
                    next.call(req).await
                }

                // Validation Bad
                Ok(None) => {
                    match req.cookie(auth::AUTH_COOKIE) {
                        Some(mut cookie) => cookie.make_removal(),
                        None => {},
                    }
                    Ok(redirect_response(state, req, LOGIN_PATH))
                },
                Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
            }
        }
    }
}

pub async fn permission_middleware(
    state: Data<AppState>,
    req: ServiceRequest,
    next: middleware::Next<BoxBody>
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    
    let path = req.path();
    if !state.permission_table.contains(path) {
        return next.call(req).await;
    }

    let user = match auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await {
        Ok(user) => user,
        Err(_) => return next.call(req).await
    };

    match state.permission_table.check_access(req.path(), user.role) {
        true => next.call(req).await,
        false => Err(actix_web::error::ErrorUnauthorized("Access Denied")),
    }
}

//
//              API
//

#[derive(Serialize)]
struct UserResponse {
    name: Option<String>,
    email: String,
    balance: f32,
    role: Role
}

#[get("/api/get_user")]
pub async fn get_user(state: Data<AppState>, req: HttpRequest) -> Result<web::Json<UserResponse>, AppError> {
    let user = auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await?;
    
    let user_response = UserResponse {
        name: user.name,
        email: user.email,
        balance: user.balance,
        role: user.role
    };
    Ok(web::Json(user_response))
}


fn product_assert_permission(product: &Product, user: &User) -> Result<(), AppError> {
    // Check if product may be modified
    if !product.flags.modifiable && user.role != Role::Admin {
        return Err(AppError::GenericError("Access Denied".to_string()));
    }

    Ok(())
}

async fn user_from_cookie(pool: &SqlitePool, req: &HttpRequest) -> Result<User, AppError> {
    let user = auth::get_user_from_cookie(pool, req.cookie(auth::AUTH_COOKIE)).await?;

    Ok(user)
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
    
    database::crud::update_product_data(&state.db, product.clone().into_row()).await?;

    if let Some(file) = form.image {
        if utils::save_img_to_disk(file, &product.id.to_string()).is_none() {
            return Err(AppError::GenericError("Product image not saved".to_string())) 
        }
    }

    let products = database::crud::get_products(&state.db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[derive(serde::Deserialize)]
struct ProductDeletionParams { id: u32 }

#[post("/api/delete_product")]
pub async fn delete_product(state: Data<AppState>, req: HttpRequest, params: web::Json<ProductDeletionParams>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    let product = get_product_from_id(&state.db, Some(params.id)).await?;

    product_assert_permission(&product, &user)?;
    database::crud::delete_product(&state.db, product.id).await?;

    let products = database::crud::get_products(&state.db).await?;

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

    let transaction = Transaction {
        user: user.id,
        products: products.clone(),
        total_price
    };
     
    database::crud::create_transaction(&state.db, transaction).await?;

    database::crud::update_user_balance(&state.db, user.id, user.balance - total_price).await?;
    
    for (product, quantity) in products {
        let new_stock = Some(product.stock.unwrap() - quantity as i32);
        database::crud::update_product_stock(&state.db, product.id, new_stock).await?;
    }

    Ok(HttpResponse::Ok())
}

//
//              Google OAuth
//

#[derive(Deserialize)]
struct AuthRequest {
    code: String,
}

#[derive(Deserialize, Debug)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct GoogleUserInfo {
    email: String,
    id: String, // Unique google_id for each user (doesn't change)
}

#[get("/api/auth/google")]
pub async fn google_login(state: Data<AppState>) -> impl Responder {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&redirect_uri={}/api/auth/google/callback&response_type=code&\
        scope=openid%20email&access_type=offline",
        state.env.google_client_id, state.env.site_domain
    );

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/api/auth/google/callback")]
pub async fn google_callback(state: Data<AppState>, query: web::Query<AuthRequest>) -> impl Responder {
    let resp: GoogleTokenResponse = state.client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", state.env.google_client_id.as_str()),
            ("client_secret", state.env.google_client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", format!("{}/api/auth/google/callback", state.env.site_domain).as_str()),
        ])
        .send().await.unwrap()
        .json().await.unwrap();

    let user_info: GoogleUserInfo = state.client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&resp.access_token)
        .send().await.unwrap()
        .json().await.unwrap();

    let mut user = crud::get_user(&state.db, None, Some(&user_info.id)).await;
    if user.is_err() {
        user = crud::create_user(&state.db, None, &user_info.email, &user_info.id).await;
    };

    let session_token = match auth::create_session(&state.db, user?.id).await {
        Ok((_, token)) => token,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Could not create session"))
    };
    let cookie = Cookie::build(auth::AUTH_COOKIE, session_token)
        .path("/")
        .http_only(true)
        .secure(false) // TODO Switch to HTTPS
        .same_site(actix_web::cookie::SameSite::Lax)
        .max_age(Duration::weeks(4)).finish();
    Ok(HttpResponse::Found()
        .append_header(("Location", utils::get_path(&state, "/")))
        .cookie(cookie)
        .finish()
    )
}

//
//      Debug
//


#[derive(serde::Deserialize)]
struct MoneyParams { amount: f32 }

#[post("/api/debug/add_money")]
pub async fn debug_add_money(state: Data<AppState>, req: HttpRequest, params: web::Json<MoneyParams>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    let new_balance = user.balance + params.amount;
    crud::update_user_balance(&state.db, user.id, new_balance).await?;

    Ok(HttpResponse::Ok())
}
