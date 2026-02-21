pub mod routes;
pub mod oauth;
pub mod products;
pub mod user;
pub mod stats;
pub mod debug;
pub mod payment;
pub mod transactions;

use actix_web::{HttpMessage, HttpRequest, HttpResponse, body::BoxBody, dev::{ServiceRequest, ServiceResponse}, middleware, web::Data};
use sqlx::SqlitePool;

use crate::{AppError, AppState, auth, database::model::UserRow, utils::{self, get_path}};

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
                    req.extensions_mut().insert(session.clone());
                    if path == LOGIN_PATH {
                        return Ok(redirect_response(state, req, "/")) 
                    };
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

pub async fn user_from_cookie(pool: &SqlitePool, req: &HttpRequest) -> Result<UserRow, AppError> {
    let user = auth::get_user_from_cookie(pool, req.cookie(auth::AUTH_COOKIE)).await?;

    Ok(user)
}
