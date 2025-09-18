use konsfekt::{database, routes, AppState};

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
    .bind(("0.0.0.0", 5656))?
    .run()
    .await
}
