use kons_coin::{auth, database::{self, crud}, AppState};

use actix_web::{body::MessageBody, dev::{ConnectionInfo, ServiceRequest, ServiceResponse}, get, middleware, post, web::{self, Data}, App, HttpMessage, HttpRequest, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUserData {
    personal_number: String,
    name: String,
}

#[post("/create-user")]
async fn create_user(state: Data<AppState>, user_data: web::Json<CreateUserData>) -> impl Responder {
    return match crud::create_user(&state.db, &user_data.name, &user_data.personal_number).await {
        Err(_) => Err(actix_web::error::ErrorConflict("User may already exist")),
        Ok(id) => Ok(id.to_string())
    }
}

#[get("/get-user")]
async fn get_user(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    let Some(session) = extensions.get::<auth::Session>() else {
        return Err(actix_web::error::ErrorUnauthorized("Could not verify session token"));
    };
    return match crud::get_user(&state.db, session.user).await {
        Ok(user) => Ok(web::Json(user)),
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Could not find user"))
    };
}

#[post("/authenticate-user")]
async fn login_user(state: Data<AppState>, req: HttpRequest, conn: ConnectionInfo) -> impl Responder {
    let Some(real_ip) = conn.realip_remote_addr() else {
        return Err(actix_web::error::ErrorForbidden("Could not find end user IP"))
    };
    // Call bankid (get orderRef)
    // Create "start url" (to start bankid client) and give it to user
    
    // Poll /collect with orderRef until user has identified
    todo!();
}

async fn session_middleware(
    state: Data<AppState>,
    req: ServiceRequest, 
    next: middleware::Next<impl MessageBody>) 
    -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let path = req.path();
    if path == "/login_user" || path == "/register_user" {
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
    let _ = dotenv::dotenv(); // Load .env file if there is one (only dev)

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
