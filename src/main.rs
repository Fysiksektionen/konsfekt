use std::{collections::HashMap, error::Error, time::Duration};

use kons_coin::{auth, database::{self, crud}, types::bankid::{CollectResponse, OrderResponse}, AppError, AppState};

use actix_web::{body::MessageBody, dev::{ConnectionInfo, ServiceRequest, ServiceResponse}, get, middleware, post, web::{self, Data}, App, HttpMessage, HttpRequest, HttpServer, Responder};
use serde::{de::DeserializeOwned, Deserialize};
use tokio::time::sleep;
use uuid::Uuid;

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
    return match crud::get_user(state, Some(session.user), None).await {
        Ok(user) => Ok(web::Json(user)),
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Could not find user"))
    };
}

#[post("/authenticate-user")]
async fn authenticate_user(state: Data<AppState>, conn: ConnectionInfo) -> impl Responder {
    let Some(real_ip) = conn.realip_remote_addr() else {
        return Err(actix_web::error::ErrorForbidden("Could not find end user IP"))
    };
    let nonce = Uuid::new_v4(); // One time order identifier
    let return_url = format!("{}/login#nonce={}", state.env_vars.frontend_url, nonce);

    let auth_json = HashMap::from([
        ("endUserIp", real_ip),
        ("returnUrl", &return_url)
    ]);
    let order_response: OrderResponse = post_bankid_api(&state, "/auth", &auth_json).await?;

    let _ = auth::create_bankid_order(&state.db, order_response.order_ref.clone(), nonce).await;

    actix_web::rt::spawn(poll_collect(state, order_response.clone()));

    let autostart_url = format!("https://app.bankid.com/?autostarttoken={}", order_response.auto_start_token);
    Ok(autostart_url)
}

#[derive(Deserialize)]
struct NoncePayload {
    nonce: String
}

#[post("/finalize_auth")]
async fn finalize_auth(state: Data<AppState>, nonce: web::Json<NoncePayload>) -> impl Responder {
    
}

async fn poll_collect(state: Data<AppState>, order_response: OrderResponse) -> Result<bool, AppError> {
    let order_ref = order_response.order_ref;
    let collect_json = HashMap::from([
        ("orderRef", order_response.order_ref.as_str())
    ]);
    loop {
        // TODO Verify bankid response authenticity
        let collect_response: CollectResponse = post_bankid_api(&state, "/collect", &collect_json).await?;
        let user = match collect_response.status.as_str() {
            "complete" => {
                match collect_response.completion_data {
                    Some(data) => crud::get_or_create_user(state, &data.user.name, &data.user.personal_number).await,
                    None => None
                }
            }
        }
        if collect_response.status == "pending" {
            sleep(Duration::from_secs(2)).await;
            continue;
        }

        if collect_response.status == "complete" && collect_response.completion_data.is_some() {
            let data = collect_response.completion_data.unwrap();
            let user = crud::get_or_create_user(state, &data.user.name, &data.user.personal_number).await?;
            auth::update_bankid_order(
                &state.db, 
                order_response.order_ref, 
                collect_response.status, 
                Some(user.id)).await;
        } else {

            return Ok(false);
        }
        auth::update_bankid_order(
            &state.db, 
            order_response.order_ref, 
            collect_response.status, 
            Some(user.id)).await;
    }

}

async fn post_bankid_api<T: DeserializeOwned>(state: &Data<AppState>, endpoint: &str, payload: &HashMap<&str, &str>) -> Result<T, actix_web::Error> {
    state.client.post(format!("{}/{}", state.env_vars.bankid_api, endpoint))
        .json(payload)
        .send().await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error when communicating with BankID's services"))?
        .json::<T>().await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not parse BankID response"))

}

async fn session_middleware(
    state: Data<AppState>,
    req: ServiceRequest, 
    next: middleware::Next<impl MessageBody>) 
    -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let path = req.path();
    if path == "/authenticate-user" {
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
            .app_data(Data::new(AppState::from(pool.clone())))
            .service(authenticate_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
