use kons_coin::{database, routes, AppState};

use actix_web::{web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState::from(pool.clone())))
            .service(routes::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
