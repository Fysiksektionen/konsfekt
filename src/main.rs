use std::env;

use actix_cors::Cors;
use actix_web::http;
use konsfekt::{auth, database, routes, AppState};
use konsfekt::{database, routes, AppState};

use actix_web::{middleware, web::Data, App, HttpServer};

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
            .wrap(middleware::from_fn(routes::session_middleware))
            .app_data(Data::new(AppState::from(pool.clone())))
            .wrap(cors)
            .service(routes::hello) // temp
            .service(routes::login) // temp
            .service(routes::google_login)
            .service(routes::google_callback)
            .service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind((if serve_over_lan { "0.0.0.0" } else { "127.0.0.1"}, 8080))?
    .run()
    .await
}
