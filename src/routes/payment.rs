use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::{self, Data}};
use sqlx::SqlitePool;
use uuid::Uuid;
use reqwest::Client;
use serde::{Deserialize, Serialize};


pub enum PaymentMethod {
    Swish,
}

pub mod swish {
    use actix_web::{HttpRequest, Responder, post, web::{self, Data}};
    use reqwest::Client;
    use sqlx::database;
    use uuid::Uuid;

    use crate::{AppError, AppState, database::{crud, model::SwishPaymentRow}, routes::user_from_cookie};


    const PAYEE_NUMBER: &str = "0123456789"; // Should be env
    const CALLBACK_URL: &str = "/payment/swish/callback"; // Remember to change post function
    const SWISH_REQUEST_URL: &str = "";

    #[derive(serde::Serialize)]
    #[allow(non_snake_case)]
    pub struct PaymentRequestObject {
        payeeAlias: String,
        amount: f32,
        currency: String,
        callbackUrl: String,
        // payeePaymentReference: String, // Vet inte vad detta är
        message: String,
        callbackIdentifier: Option<String>, // Sätt att skydda oss
    }

    impl PaymentRequestObject {
        pub fn new(state: &Data<AppState>, amount: f32) -> Self {
            PaymentRequestObject {
                payeeAlias: String::from(PAYEE_NUMBER),
                amount,
                currency: String::from("SEK"),
                callbackUrl: String::from(state.env.site_domain.clone() + CALLBACK_URL),
                // payeePaymentReference: String::from("Vet inte vad det här är?"),
                message: String::from("Konsfekt Betalning"),
                callbackIdentifier: None
            }
        }
    }
    
    #[derive(serde::Deserialize)]
    #[allow(non_snake_case)]
    pub struct PaymentCallback {
        id: String,
        payeePaymentReference: String,
        paymentReference: String,
        callbackUrl: String,
        payerAlias: String,
        payeeAlias: String,
        amount: String,
        currency: String,
        message: String,
        status: String,
        dateCreated: String,
        datePaid: String,
        errorCode: Option<String>,
        errorMessage: Option<String>,
    }

    pub struct PaymentRequest {
        id: Uuid,
        token: String,
        location: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, sqlx::Type)]
    #[serde(rename_all = "lowercase")]
    #[sqlx(type_name = "swish_status", rename_all = "lowercase")]
    pub enum Status {
        Pending, Paid
    }

    async fn create_request(state: &Data<AppState>, amount: f32) -> Result<PaymentRequest, AppError> {
        let id: Uuid = Uuid::new_v4();
        let pro = PaymentRequestObject::new(state, amount);

        // Skicka till swish
        let response = state.client.post(SWISH_REQUEST_URL)
            .json(&pro)
            .send().await?;

        if response.status() == 201 {
            return match (
                response.headers().get("PaymentRequestToken"),
                response.headers().get("Location")
            ) {
                (Some(token), Some(location)) => Ok(PaymentRequest {
                    id: id,
                    token: String::from(token.to_str()?),
                    location: String::from(location.to_str()?)
                }),
                _ => Err(AppError::SwishError(String::from("Swish is bad at coding, idk"))),
            }
        }

        return Err(AppError::SwishError(String::from(format!("Bad Swish Request, got status code [{}]", response.status()))))

    }

    async fn handle_callback(payment_callback: PaymentCallback) -> Result<(), AppError>{
        println!("CALLBACK!!!");
        
        Ok(())
    }


    #[derive(serde::Deserialize)]
    struct CreatePaymentRequestQuery { amount: f32 }

    #[derive(serde::Serialize)]
    struct CreatePaymentRequestResponse { token: String }

    #[post("/payment/swish/create_payment_request")]
    pub async fn create_payment_request(state: Data<AppState>, req: HttpRequest, query: web::Query<CreatePaymentRequestQuery>) -> Result<impl Responder, AppError> {
        let user = user_from_cookie(&state.db, &req).await?;
        
        if query.amount < 30.0 {
            return Err(AppError::BadRequest(String::from("amount < 30 kr")))
        }

        let payment_request = create_request(&state, query.amount).await?;
        let _ = crud::create_payment_request(&state.db, SwishPaymentRow {
            id: payment_request.id,
            user: user.id,
            status: Status::Pending,
            token: payment_request.token.clone(),
            location: payment_request.location,
        }).await?;

        Ok(web::Json(CreatePaymentRequestResponse {
            token: payment_request.token
        }))
    }

    #[post("/payment/swish/callback")] // Remember to change CALLBACK_URL
    pub async fn swish_callback(callback: web::Json<PaymentCallback>) -> Result<(), AppError> {
        handle_callback(callback.into_inner()).await?;
        Ok(())
    }


}


