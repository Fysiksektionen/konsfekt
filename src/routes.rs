use core::sync;

use actix_web::{body::BoxBody, cookie::Cookie, dev::{ServiceRequest, ServiceResponse}, get, middleware, web::{self, Data}, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::{auth, database::{crud, model::User}, utils::{self, get_path}, AppError, AppState};

const LOGIN_PATH: &str = "/login";
const PATH_WHITELIST: [&str; 2] = [
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
    
    println!("{}", path);
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

//
//              API
//

#[derive(Serialize)]
struct UserResponse {
    name: Option<String>,
    email: String,
    balance: f32,
}

#[get("/api/get_user")]
pub async fn get_user(state: Data<AppState>, req: HttpRequest) -> Result<web::Json<UserResponse>, AppError> {

    let user = auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await?;
    
    let user_response = UserResponse {
        name: user.name,
        email: user.email,
        balance: user.balance,
    };

    Ok(web::Json(user_response))
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
