use actix_cors::Cors;
use actix_web::{http, middleware::DefaultHeaders, web::scope};
use konsfekt::{database, routes, AppState, EnvironmentVariables};

use actix_web::{middleware, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = EnvironmentVariables::new();

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    
    if env.static_frontend {
        println!("Web server running at {}", env.site_domain);
    } else {
        print!("Backend running at {}\nFrontend needs to be served separately\n", env.site_domain)
    }

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
            .wrap(middleware::from_fn(routes::permission_middleware))
            .app_data(Data::new(AppState::from(pool.clone(), env_clone.clone())))
            .wrap(cors)

            // Google Auth
            .service(routes::google_login)
            .service(routes::google_callback)

            // User API
            .service(routes::get_user)

            // Product API
            .service(routes::create_product)
            .service(routes::get_products)
            .service(routes::update_product)
            .service(routes::update_stock)
            .service(routes::delete_product)

            .service(routes::buy_product)

            // Uploads
            .service(scope("/uploads")
                .wrap(DefaultHeaders::new().add(("Cache-Control", "public, max-age=0, must-revalidate")))
                .service(actix_files::Files::new("", "./db/uploads")));

        if env.static_frontend {
            app.service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
        } else {
            app
        }
    })
    .bind((if env.is_debug { "127.0.0.1" } else { "0.0.0.0" }, 8080))?
    .run()
    .await
}
