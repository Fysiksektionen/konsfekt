use konsfekt::{auth, database, routes, AppState};

use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware, web::Data, App, HttpMessage, HttpServer};

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

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::from_fn(session_middleware))
            .app_data(Data::new(AppState::from(pool.clone())))
            .service(routes::hello)
            .service(routes::google_login)
            .service(routes::google_callback)
            .service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
