use konsfekt::{database, routes, AppState};

use actix_web::{middleware, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    HttpServer::new(move || {
        App::new()
            // .wrap(auth_redirect::AuthRedirect)
            .wrap(middleware::from_fn(routes::session_middleware))
            .app_data(Data::new(AppState::from(pool.clone())))
            .service(routes::hello) // temp
            .service(routes::login) // temp
            .service(routes::google_login)
            .service(routes::google_callback)
            .service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
