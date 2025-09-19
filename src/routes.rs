
use std::env;

use actix_web::{get, web, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
pub async fn google_login() -> impl Responder {
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").unwrap();
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&redirect_uri={}&response_type=code&\
        scope=openid%20email&access_type=offline",
        client_id, redirect_uri
    );

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/auth/google/callback")]
pub async fn google_callback(query: web::Query<AuthRequest>) -> impl Responder {
    let client = Client::new();
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").unwrap();

    let resp: GoogleTokenResponse = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send().await.unwrap()
        .json().await.unwrap();

    let user_info: GoogleUserInfo = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&resp.access_token)
        .send().await.unwrap()
        .json().await.unwrap();

    // Temp, redirect and create new session
    return HttpResponse::Ok().json(user_info)

}