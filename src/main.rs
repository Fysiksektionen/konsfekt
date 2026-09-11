use std::{format, time::Duration};

use actix_cors::Cors;
use actix_web::{http, middleware::DefaultHeaders, web::scope};
use clap::Parser;
use konsfekt::{database, routes, AppState, EnvironmentVariables, args};

use actix_web::{middleware, web::Data, App, HttpServer};
use sqlx::{Sqlite, SqlitePool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let args = args::Args::parse();
    let env = EnvironmentVariables::from_args(args);

    create_logger(&env);

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    
    if env.static_frontend {
        log::info!("Web server running at {}", env.site_domain);
    } else {
        log::info!("Backend running at {}", env.site_domain);
        log::info!("Frontend needs to be served separately")
    }

    if env.backups_enabled {
        log::info!("Creating backup...");
        let (db, uploads) = database::backup::create_backup(&pool)
            .await
            .expect("Could not create backup");
        log::info!(
            "Successfully created database backup \"{}\" and uploads backup \"{}\"",
            db, uploads
        );

        start_backup_interval(&env, pool.clone());
    }

    let env_clone = env.clone();
    HttpServer::new(move || create_http(env_clone.clone(), pool.clone()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

fn start_backup_interval(env: &EnvironmentVariables, pool: SqlitePool) {
    let sleep_time: u64 = env.backup_interval.second() as u64
                        + env.backup_interval.minute() as u64   * 60 
                        + env.backup_interval.hour()   as u64   * 60 * 60;

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(sleep_time)).await;    
             
            match database::backup::create_backup(&pool).await {
                Ok((db, uploads)) => log::info!(
                    "Successfully created database backup \"{}\" and uploads backup \"{}\"",
                    db,
                    uploads
                ),
                Err(_) => log::error!("Failed to create backup.")
            };
        }



    });
}

fn create_logger(env: &EnvironmentVariables) {
    
    let mut builder = env_logger::Builder::from_default_env();

    // Log error, warn and info
    builder.filter_module("konsfekt", log::LevelFilter::Info);

    // Log error, warn, info and debug
    if env.is_debug {
        builder.filter_module("konsfekt", log::LevelFilter::Debug);
        builder.filter_module("reqwest", log::LevelFilter::Debug);
        builder.filter_module("hyper", log::LevelFilter::Debug);
    }

    builder.init();
}

fn create_http(env: EnvironmentVariables, pool: sqlx::Pool<Sqlite>) -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, Error = actix_web::Error, InitError = ()>> {
    let mut cors = Cors::default()
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE, 
                http::header::AUTHORIZATION, 
            ]);
    if !env.static_frontend {
        cors = cors.allowed_origin(&env.clone().frontend_url);
    }
    cors = cors
        .allowed_origin("tauri://localhost")
        .allowed_origin("http://tauri.localhost");
    let mut app = App::new()
        .wrap(middleware::from_fn(routes::session_middleware))
        .app_data(Data::new(AppState::from(pool.clone(), env.clone())))
        .app_data(actix_web::web::JsonConfig::default().error_handler(|err, req| {
            log::warn!("JSON body error on {}: {}", req.path(), err);
            actix_web::error::InternalError::from_response(err, actix_web::HttpResponse::BadRequest().finish()).into()
        }))
        .wrap(cors)
        .wrap(middleware::from_fn(routes::api_logging_middleware))

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
        .service(routes::user::unlink_transactions)
        .service(routes::user::set_user_flags)

        // Transaction API
        .service(routes::transactions::get_transactions)
        .service(routes::transactions::get_detailed_transaction)

        // Product API
        .service(routes::products::create_product)
        .service(routes::products::get_products)
        .service(routes::products::update_product)
        .service(routes::products::delete_product)

        .service(routes::products::buy_products)
        .service(routes::products::buy_single_product)
        .service(routes::products::undo_purchase)
        .service(routes::products::mark_sold_out)
        
        // Swish API
        .service(routes::payment::swish::create_payment_request)
        .service(routes::payment::swish::swish_callback)
        .service(routes::payment::swish::check_status)
        .service(routes::payment::swish::get_qr_code)

        // Stats API
        .service(routes::stats::best_selling_product)
        .service(routes::stats::purchases)
        .service(routes::stats::customers)
        .service(routes::stats::deposits)

        // Backup
        .service(routes::backup::create_backup)

        // Uploads
        .service(scope("/uploads")
            .wrap(DefaultHeaders::new().add(("Cache-Control", "public, max-age=0, must-revalidate")))
            .service(actix_files::Files::new("", "./db/uploads")));

    if env.is_debug {
        app = app.service(routes::debug::add_money);
    }

    if env.static_frontend {
        app.service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
            .default_service(actix_web::web::get().to(|| async {
                actix_files::NamedFile::open("./frontend/build/index.html")
            }))
    } else {
        app
    }
}
