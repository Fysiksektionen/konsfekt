use std::env;
use actix_cors::Cors;
use konsfekt::{auth, database, routes, AppState};

use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http, middleware, web::Data, App, HttpMessage, HttpServer};

async fn session_middleware(
    state: Data<AppState>,
    req: ServiceRequest, 
    next: middleware::Next<impl MessageBody>) 
    -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let path = req.path();
    if path.starts_with("/auth/google") {
        return next.call(req).await;
    }
    match auth::parse_auth_cookie(req.cookie(auth::AUTH_COOKIE)) {
        None => return Err(actix_web::error::ErrorUnauthorized("Could not find session token")),
        Some(token) => {
            return match auth::validate_session(&state.db, token).await {
                Ok(Some(session)) => {
                    req.extensions_mut().insert(session);
                    next.call(req).await
                },
                Ok(None) => Err(actix_web::error::ErrorUnauthorized("Session token unauthorized")),
                Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();

    let is_debug = cfg!(debug_assertions);
    let serve_over_lan = env::var("LAN_SERVER").unwrap_or("false".into()).parse::<bool>().unwrap_or(false);
     
    let ip = if serve_over_lan { 
        local_ip_address::local_ip().expect("Failed to get ip address").to_string()
    } else {
        "127.0.0.1".to_string()
    };
    let origin = format!("http://{ip}:8080");

    if is_debug {
        println!("Backend running at {origin}");
    }

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    HttpServer::new(move || {
        let mut cors = Cors::default()
                .supports_credentials()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    http::header::CONTENT_TYPE, 
                    http::header::AUTHORIZATION, 
                ]);
        if is_debug {
            cors = cors
                .allowed_origin("http://localhost:5173")
                .allowed_origin("http://127.0.0.1:5173")
                .allowed_origin(&origin);
        }
        App::new()
            .wrap(middleware::from_fn(session_middleware))
            .app_data(Data::new(AppState::from(pool.clone())))
            .wrap(cors)
            .service(routes::hello)
            .service(routes::google_login)
            .service(routes::google_callback)
            .service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind((if serve_over_lan { "0.0.0.0" } else { "127.0.0.1"}, 8080))?
    .run()
    .await
}
