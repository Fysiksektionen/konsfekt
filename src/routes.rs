use actix_web::{get, web::{self, Data}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::AppState;

//
//             API
//

#[get("/api")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello backend!")
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

#[get("/auth/google/login")]
pub async fn google_login(state: Data<AppState>) -> impl Responder {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&redirect_uri={}&response_type=code&\
        scope=openid%20email&access_type=offline",
        state.env_vars.google_client_id, state.env_vars.google_redirect_uri
    );

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/auth/google/callback")]
pub async fn google_callback(state: Data<AppState>, query: web::Query<AuthRequest>) -> impl Responder {

    let resp: GoogleTokenResponse = state.client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", state.env_vars.google_client_id.as_str()),
            ("client_secret", state.env_vars.google_client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", state.env_vars.google_redirect_uri.as_str()),
        ])
        .send().await.unwrap()
        .json().await.unwrap();

    let user_info: GoogleUserInfo = state.client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&resp.access_token)
        .send().await.unwrap()
        .json().await.unwrap();

    // Temp, redirect and create new session
    return HttpResponse::Ok().json(user_info)
}
