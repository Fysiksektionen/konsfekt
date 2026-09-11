use actix_web::{HttpMessage, HttpRequest, HttpResponse, cookie::Cookie, get, post, web::{self, Data}};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use time::Duration;

use crate::{AppState, auth::{self, Session}, database::crud::{self, EmailSwitch}, error::{ApiResult, AppError, ClientError, DatabaseError, GenericError}, return_err, routes::CurrentUser, utils};

//
//              Google OAuth
//

pub const LOGIN_STATE_COOKIE: &str = "login-state";

/// Sent to /api/auth/google/callback
#[derive(Deserialize)]
struct GoogleCallbackQuery {
    code: Option<String>,
    state: Option<String> // Email switch token
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

impl GoogleUserInfo {
    pub async fn get_from_google(state: Data<AppState>, google_code: &str) -> Result<Self, AppError> {
        let resp: GoogleTokenResponse = state.client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", state.env.google_client_id.as_str()),
                ("client_secret", state.env.google_client_secret.as_str()),
                ("code", google_code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", format!("{}/api/auth/google/callback", state.env.site_domain).as_str()),
            ])
            .send().await.map_err(ClientError::from)?
            .json().await.map_err(ClientError::from)?;

        let user_info: GoogleUserInfo = state.client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(&resp.access_token)
            .send().await.map_err(ClientError::from)?
            .json().await.map_err(ClientError::from)?;

        return Ok(user_info);
    }

}

fn get_google_auth_url(state: &Data<AppState>) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&redirect_uri={}/api/auth/google/callback&response_type=code&\
        scope=openid%20email&access_type=online",
        state.env.google_client_id, state.env.site_domain
    )
}

#[get("/api/auth/google")]
pub async fn google_login(state: Data<AppState>) -> ApiResult<HttpResponse> {
    let login_state = utils::gen_secure_random_str().ok_or(GenericError::new("Could not generate login state"))?;
    
    let mut auth_url = get_google_auth_url(&state);
    auth_url.push_str("&state=");
    auth_url.push_str(&login_state);

    let cookie = Cookie::build(LOGIN_STATE_COOKIE, login_state)
        .path("/api/auth")
        .http_only(true)
        .secure(state.env.is_running_https)
        .same_site(actix_web::cookie::SameSite::Lax)
        .max_age(Duration::seconds(60)).finish();

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .cookie(cookie)
        .finish())
}

#[get("/api/auth/google/callback")]
pub async fn google_callback(state: Data<AppState>, req: HttpRequest, query: web::Query<GoogleCallbackQuery>) -> ApiResult<HttpResponse> {
    let Some(google_code) = &query.code else {
        return_err!(actix_web::error::ErrorUnauthorized("Google code not found"));
    };
    let user_info = GoogleUserInfo::get_from_google(state.clone(), &google_code).await?;
    
    let Some(auth_state) = &query.state else {
        return_err!(actix_web::error::ErrorUnauthorized("OAuth state not found"));
    };
    if let Some(email_switch) = crud::get_email_switch(&state.db, &auth_state).await? {
        match handle_email_switch(&state.db, &email_switch, user_info).await? {
            EmailSwitchOutcome::Success => { return create_session_response(state, req, email_switch.user_id).await },
            outcome => { return_err!(actix_web::error::ErrorForbidden(format!("Email switch not successfull {:?}", outcome))); }
        }
    }

    let Some(cookie) = req.cookie(LOGIN_STATE_COOKIE).map(|c| c.value().to_string()) else {
        return_err!(actix_web::error::ErrorUnauthorized("Login state cookie not found"));
    };
    if cookie != *auth_state {
        return_err!(actix_web::error::ErrorUnauthorized("Login state's does not match"));
    }

    let mut user = crud::get_user(&state.db, None, Some(&user_info.id)).await;
    if user.is_err() {
        user = crud::create_user(&state.db, None, &user_info.email, &user_info.id).await;
    };

    return create_session_response(state, req, user?.id).await;
}

#[derive(Debug)]
pub enum EmailSwitchOutcome {
    Success,
    Expired,
    EmailTaken,
    NoOp
}

fn try_make_removal_on_cookie(req: HttpRequest, cookie_name: &str) -> Option<Cookie<'_>> {
    if let Some(mut cookie) = req.cookie(cookie_name) {
        cookie.make_removal();
        return Some(cookie);
    }
    return None;
}

async fn handle_email_switch(pool: &SqlitePool, email_switch: &EmailSwitch, user_info: GoogleUserInfo) -> Result<EmailSwitchOutcome, DatabaseError> {
    if email_switch.expired && !email_switch.completed {
        crud::remove_email_switch(pool, &email_switch.token).await?;
        return Ok(EmailSwitchOutcome::Expired);
    }
    if let Ok(existing_user) = crud::get_user(pool, None, Some(&user_info.id.to_string())).await {
        if existing_user.id != email_switch.user_id {
            return Ok(EmailSwitchOutcome::EmailTaken)
        } else {
            return Ok(EmailSwitchOutcome::NoOp);
        }
    }
    if email_switch.completed {
        return Ok(EmailSwitchOutcome::NoOp);
    }
    
    // Logout user from all devices
    crud::invalidate_all_user_sessions(pool, email_switch.user_id).await?;

    crud::finalize_email_switch(pool, email_switch, &user_info.email, &user_info.id).await?;

    return Ok(EmailSwitchOutcome::Success);
}

async fn create_session_response(state: Data<AppState>, req: HttpRequest, user_id: u32) -> ApiResult<HttpResponse> {
    let session_token = match auth::create_session(&state.db, user_id).await {
        Ok((_, token)) => token,
        Err(_) => { return_err!(actix_web::error::ErrorInternalServerError("Could not create session")); },
    };
    let cookie = Cookie::build(auth::SESSION_COOKIE, session_token)
        .path("/")
        .http_only(true)
        .secure(state.env.is_running_https)
        .same_site(actix_web::cookie::SameSite::Strict)
        .max_age(Duration::weeks(2)).finish();

    let mut resp_builder = HttpResponse::Found();
    resp_builder
        .append_header(("Location", utils::get_path(&state, "/")))
        .cookie(cookie);
    if let Some(login_state_cookie) = try_make_removal_on_cookie(req, LOGIN_STATE_COOKIE) {
        resp_builder.cookie(login_state_cookie);
    }
    Ok(resp_builder.finish())
}

#[get("/api/auth/logout")]
pub async fn logout(state: Data<AppState>, req: HttpRequest) -> ApiResult<HttpResponse> {
    let extensions = req.extensions();
    let Some(session) = extensions.get::<Session>() else {
        return_err!(actix_web::error::ErrorInternalServerError("Could not find session to remove for logged in user"));
    };
    auth::invalidate_session(&state.db, session).await?;

    // Redirect to /login and remove session cookie
    let mut resp_builder = HttpResponse::Found();
    resp_builder.append_header(("Location", utils::get_path(&state, "/login")));
    if let Some(session_cookie) = try_make_removal_on_cookie(req.clone(), auth::SESSION_COOKIE) {
        resp_builder.cookie(session_cookie);
    }
    Ok(resp_builder.finish())
}

#[post("/api/auth/change_email")]
pub async fn change_email(state: Data<AppState>, current_user: CurrentUser) -> ApiResult<HttpResponse> {
    let user = current_user.into_row();
    let email_switch_token = utils::gen_secure_random_str().ok_or(GenericError::new("Could not generate email switch token"))?;

    let mut auth_url = get_google_auth_url(&state);
    auth_url.push_str("&state=");
    auth_url.push_str(&email_switch_token);

    crud::initiate_email_switch(&state.db, user.id, &email_switch_token).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish())
}
