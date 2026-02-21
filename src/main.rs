use actix_cors::Cors;
use actix_web::{http, middleware::DefaultHeaders, web::scope};
use konsfekt::{database, routes, AppState, EnvironmentVariables};

use actix_web::{middleware, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = EnvironmentVariables::new();

    let mut builder = env_logger::Builder::from_default_env();

    builder.filter_module("konsfekt", log::LevelFilter::Error);
    builder.filter_module("konsfekt", log::LevelFilter::Warn);
    builder.filter_module("konsfekt", log::LevelFilter::Info);
    builder.filter_module("konsfekt", log::LevelFilter::Trace);

    if env.is_debug {
        builder.filter_module("konsfekt", log::LevelFilter::Debug);
    }

    builder.init();

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    
    if env.static_frontend {
        log::info!("Web server running at {}", env.site_domain);
    } else {
        log::info!("Backend running at {}", env.site_domain);
        log::info!("Frontend needs to be served separately")
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
        let mut app = App::new()
            .wrap(middleware::from_fn(routes::session_middleware))
            .wrap(middleware::from_fn(routes::permission_middleware))
            .app_data(Data::new(AppState::from(pool.clone(), env_clone.clone())))
            .wrap(cors)

            // Google Auth
            .service(routes::oauth::google_login)
            .service(routes::oauth::google_callback)
            .service(routes::oauth::logout)
            .service(routes::oauth::change_email)

            // User API
            .service(routes::user::get_user)
            .service(routes::user::delete_user)
            .service(routes::user::update_user)
            .service(routes::user::set_username)
            .service(routes::user::get_users)
            .service(routes::user::get_transactions)
            .service(routes::user::get_detailed_transaction)

            // Product API
            .service(routes::products::create_product)
            .service(routes::products::get_products)
            .service(routes::products::update_product)
            .service(routes::products::delete_product)

            .service(routes::products::buy_products)
            .service(routes::products::buy_single_product)
            .service(routes::products::undo_transaction)
            .service(routes::products::mark_sold_out)

            // Stats API
            .service(routes::stats::best_selling_product)
            .service(routes::stats::purchases)
            .service(routes::stats::customers)
            .service(routes::stats::deposits)

            // Uploads
            .service(scope("/uploads")
                .wrap(DefaultHeaders::new().add(("Cache-Control", "public, max-age=0, must-revalidate")))
                .service(actix_files::Files::new("", "./db/uploads")));

        if env.is_debug {
            app = app.service(routes::debug::add_money);
        }

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
