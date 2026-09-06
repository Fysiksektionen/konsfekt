pub mod routes;
pub mod oauth;
pub mod products;
pub mod user;
pub mod stats;
pub mod debug;
pub mod payment;
pub mod transactions;

use std::pin::Pin;

use actix_web::{FromRequest, HttpMessage, HttpRequest, HttpResponse, body::BoxBody, dev::{ServiceRequest, ServiceResponse}, middleware, web::{self, Data}};
use sqlx::SqlitePool;

use crate::{AppState, Role, auth, database::model::UserRow, error::{ApiResult, AppError}, return_err, utils::{self, get_path}};

const LOGIN_PATH: &str = "/login";
const PATH_WHITELIST: [&str; 4] = [
    LOGIN_PATH,
    "/api/auth/google",
    "/api/auth/google/callback",
    payment::swish::CALLBACK_URL,
];

//
//      CurrentUser Extractor
//

struct CurrentUser {
    user: UserRow,
}

impl CurrentUser {

    pub fn require_role(self, role: Role) -> ApiResult<Self> {
        if self.user.role >= role {
            return Ok(self);
        };

        return_err!(actix_web::error::ErrorForbidden("User doesn't have the requierd role for this request."));
    }

    pub fn into_row(self) -> UserRow {
        self.user
    }

}

impl FromRequest for CurrentUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<
        dyn Future<Output = ApiResult<Self>>
    >>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {

        let state = req
            .app_data::<web::Data<AppState>>()
            .cloned();
        
        let cookie = req.cookie(auth::AUTH_COOKIE).clone();

        Box::pin(async move {
            let state = state.ok_or_else(|| {
                actix_web::error::ErrorInternalServerError("Cant get AppState")
            })?;

            let cookie = cookie.ok_or_else(|| {
                actix_web::error::ErrorUnauthorized("Not authenticated")
            })?;

            let user = auth::get_user_from_cookie(
                &state.db,
                Some(cookie),
            )
            .await?;

            Ok(CurrentUser { user })
        })
    }
}


//
//          Helper Functions
//

pub async fn user_from_cookie(pool: &SqlitePool, req: &HttpRequest) -> Result<UserRow, AppError> {
    let user = auth::get_user_from_cookie(pool, req.cookie(auth::AUTH_COOKIE)).await?;

    Ok(user)
}


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

    if PATH_WHITELIST.contains(&path) || path.starts_with("/_app/") || path.starts_with("/uploads/") {
        return next.call(req).await;
    }
    
    match auth::parse_auth_cookie(req.cookie(auth::AUTH_COOKIE)) {
        // Cookie not found
        None => {
            if path.starts_with("/api/") {
                Err(actix_web::error::ErrorUnauthorized("No cookie found"))
            } else if path != get_path(&state, LOGIN_PATH) {
                Ok(redirect_response(state, req, LOGIN_PATH))
            } else {
                next.call(req).await
            }
        },
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
                    if path.starts_with("/api/") {
                        Err(actix_web::error::ErrorUnauthorized("Could not validate session"))
                    } else {
                        Ok(redirect_response(state, req, LOGIN_PATH))
                    }
                },
                Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
            }
        }
    }
}

/// Logs `/api/*` requests at debug level; everything else (static frontend, uploads) is never logged.
pub async fn api_logging_middleware<B: actix_web::body::MessageBody>(
    req: ServiceRequest,
    next: middleware::Next<B>
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let path = req.path().to_owned();
    if !path.starts_with("/api/") {
        return next.call(req).await;
    }

    let method = req.method().clone();
    let start = std::time::Instant::now();
    let res = next.call(req).await?;
    log::debug!("{method} {path} {} {:?}", res.status(), start.elapsed());
    Ok(res)
}