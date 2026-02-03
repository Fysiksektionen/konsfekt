use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, cookie::Cookie, get, web::{self, Data}};
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::{AppError, AppState, auth::{self, Session}, database::crud, routes::user_from_cookie, utils};

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
    // refresh_token: Option<String>
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
        scope=openid%20email&access_type=online",
        state.env.google_client_id, state.env.site_domain
    );

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/api/auth/google/callback")]
pub async fn google_callback(state: Data<AppState>, req: HttpRequest, query: web::Query<AuthRequest>) -> Result<impl Responder, AppError> {
    let resp: GoogleTokenResponse = state.client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", state.env.google_client_id.as_str()),
            ("client_secret", state.env.google_client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", format!("{}/api/auth/google/callback", state.env.site_domain).as_str()),
        ])
        .send().await?
        .json().await?;

    let user_info: GoogleUserInfo = state.client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&resp.access_token)
        .send().await?
        .json().await?;

    if let Ok(logged_in_user) = user_from_cookie(&state.db, &req).await {
        if crud::email_switch_exists(&state.db, logged_in_user.id).await? {
            crud::finalize_email_switch(&state.db, logged_in_user.id, &user_info.email, &user_info.id).await?;
        }
    }

    let mut user = crud::get_user(&state.db, None, Some(&user_info.id)).await;
    if user.is_err() {
        user = crud::create_user(&state.db, None, &user_info.email, &user_info.id).await;
    };

    return create_session_response(state, user?.id).await;
}

async fn create_session_response(state: Data<AppState>, user_id: u32) -> Result<impl Responder, AppError> {
    let session_token = match auth::create_session(&state.db, user_id).await {
        Ok((_, token)) => token,
        Err(_) => return Err(AppError::ActixError(actix_web::error::ErrorInternalServerError("Could not create session")))
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

#[get("/api/auth/logout")]
pub async fn logout(state: Data<AppState>, req: HttpRequest) -> Result<impl Responder, AppError> {
    let extensions = req.extensions();
    let session = extensions.get::<Session>().ok_or_else(||
        AppError::ActixError(actix_web::error::ErrorInternalServerError("Could not find session to remove for logged in user"))
    )?;
    auth::invalidate_session(&state.db, session).await?;
    let mut cookie = req.cookie(auth::AUTH_COOKIE).ok_or_else(||
        AppError::ActixError(actix_web::error::ErrorInternalServerError("Could not find cookie"))
    )?;

    cookie.make_removal();
    
    Ok(HttpResponse::Found()
        .append_header(("Location", utils::get_path(&state, "/login")))
        .cookie(cookie)
        .finish()
    )
}

#[get("/api/auth/change_email")]
pub async fn change_email(state: Data<AppState>, req: HttpRequest) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    crud::initiate_email_switch(&state.db, user.id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", utils::get_path(&state, "/api/auth/google")))
        .finish())
}
