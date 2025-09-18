use std::{collections::HashMap, time::Duration};

use kons_coin::{auth, database::{self, crud}, types::bankid::{CollectResponse, OrderResponse}, AppError, AppState};

use actix_web::{body::MessageBody, dev::{ConnectionInfo, ServiceRequest, ServiceResponse}, get, middleware, post, web::{self, Data}, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;

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

#[derive(Serialize)]
#[serde(transparent)]
struct SessionResponse {
    token: String,
}

#[post("/finalize-auth")]
async fn finalize_auth(state: Data<AppState>, nonce: web::Json<NoncePayload>) -> impl Responder {
    let order = auth::get_bankid_order(&state.db, None, Some(nonce.nonce)).await?;
    match order.status.as_str() {
        "complete" => {},
        "failed" => {
            return Err(actix_web::error::ErrorUnauthorized("Bankid authentication failed"));
        }
        "pending" => { 
            return HttpResponse::Accepted()
                .content_type("application/json")
                .body(r#"{"status":"pending"}"#);
        },
    };

    // if let Some(user_id) = order.user_id {
    //     return match auth::create_session(&state.db, user_id).await? {
    //         Some((_, token)) => Ok(web::Json(SessionResponse { token })),
    //         None => Err(actix_web::error::ErrorInternalServerError("Could not create session"))
    //     };
    // };
    // Err(actix_web::error::ErrorInternalServerError("Could not find user"))
}

async fn poll_collect(state: Data<AppState>, order_response: OrderResponse) -> Result<bool, AppError> {
    let order_ref = order_response.order_ref.clone();
    let collect_json = HashMap::from([
        ("orderRef", order_ref.as_str())
    ]);
    loop {
        // TODO Verify bankid response authenticity
        let collect_response: CollectResponse = post_bankid_api(&state, "/collect", &collect_json).await?;
        let user = collect_response.get_user(&state).await?;

        auth::update_bankid_order(
            &state.db, 
            order_ref.clone(), 
            collect_response.status.clone(), 
            user.map(|u| u.id)).await;

        match collect_response.status.as_str() {
            "complete" => return Ok(true),
            "pending" => {
                sleep(Duration::from_secs(2)).await;
                continue;
            },
            _ => return Ok(false) // failed
        }
    }

}

async fn post_bankid_api<T: DeserializeOwned>(state: &Data<AppState>, endpoint: &str, payload: &HashMap<&str, &str>) -> Result<T, AppError> {
    Ok(state.client.post(format!("{}/{}", state.env_vars.bankid_api, endpoint))
        .json(payload)
        .send().await?
        .json::<T>().await?)
}

async fn session_middleware(
    state: Data<AppState>,
    req: ServiceRequest, 
    next: middleware::Next<impl MessageBody>) 
    -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let path = req.path();
    if path == "/authenticate-user" || path == "/finalize-auth" {
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
