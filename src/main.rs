use actix_cors::Cors;
use actix_web::http;
use konsfekt::{database, routes, AppState, EnvironmentVariables};

use actix_web::{middleware, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = EnvironmentVariables::new();

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");

    println!("Server running on {}", env.site_domain);

    let env_clone = env.clone(); // To be used in closure
    HttpServer::new(move || {
        let mut cors = Cors::default()
                .supports_credentials()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    http::header::CONTENT_TYPE, 
                    http::header::AUTHORIZATION, 
                ]);
        if env.is_debug {
            cors = cors.allowed_origin(&env_clone.frontend_url);
        }
        let app = App::new()
            .wrap(middleware::from_fn(routes::session_middleware))
            .app_data(Data::new(AppState::from(pool.clone(), env_clone.clone())))
            .wrap(cors)
            .service(routes::google_login)
            .service(routes::google_callback);
        if env.is_debug {
            app 
        } else {
            app.service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
        }
    })
    .bind((if env.is_debug { "127.0.0.1" } else { "0.0.0.0" }, 8080))?
    .run()
    .await
}
