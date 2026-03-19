pub enum PaymentMethod {
    Swish,
}

pub mod swish {
    use actix_web::{HttpRequest, post, web::{self, Data}};
    use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
    use uuid::Uuid;

    use crate::{AppState, database::{crud, model::SwishPaymentRow}, error::{ApiResult, AppError, ClientError, GenericError, SwishError, SwishErrorResponse}, return_err, routes::user_from_cookie};


    const PAYEE_NUMBER: &str = "1234679304"; // Should be env
    const CALLBACK_URL: &str = "/api/payment/swish/callback"; // Remember to change post function
    const SWISH_REQUEST_URL: &str = "https://mss.cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/";

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

    #[derive(serde::Deserialize, Debug)]
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

    async fn create_request(state: &Data<AppState>, amount: f32) -> Result<web::Json<PaymentRequest>, AppError> {
        let id: Uuid = Uuid::new_v4();
        let pro = PaymentRequestObject::new(state, amount);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = state.client
            .put(format!("{}{}", SWISH_REQUEST_URL, id.simple().to_string().to_uppercase()))
            .headers(headers)
            .json(&pro)
            .send().await.map_err(ClientError::from)?;

        let status = response.status();

        if status == 201 {
            let token = response.headers().get("PaymentRequestToken").map(|t| t.to_str().ok()).flatten();
            let location = response.headers().get("Location").map(|l| l.to_str().ok()).flatten();
            return match (token, location) {
                (Some(token), Some(location)) => {
                    Ok(web::Json(PaymentRequest {
                        id,
                        token: String::from(token),
                        location: String::from(location),
                    }))
                },
                _ => Err(GenericError::new("Could not find/parse Swish response token or location").into())
            }
        }
        Err(SwishErrorResponse::to_error(response).await.into())
    }

    async fn handle_callback(payment_callback: PaymentCallback) -> Result<(), AppError> {
        println!("CALLBACK!!!");
        Ok(())
    }


    #[derive(serde::Deserialize)]
    struct CreatePaymentRequestQuery { amount: f32 }

    #[derive(serde::Serialize)]
    struct CreatePaymentRequestResponse { token: String }

    #[post("/api/payment/swish/create_payment_request")]
    pub async fn create_payment_request(state: Data<AppState>, req: HttpRequest, query: web::Query<CreatePaymentRequestQuery>) -> ApiResult<web::Json<CreatePaymentRequestResponse>> {
        let user = user_from_cookie(&state.db, &req).await?;

        if query.amount < 30.0 {
            return_err!(actix_web::error::ErrorBadRequest("amount < 30 kr"));
        }

        let payment_request = create_request(&state, query.amount).await?;
        let _ = crud::create_payment_request(&state.db, SwishPaymentRow {
            id: payment_request.id,
            user: user.id,
            status: Status::Pending,
            token: payment_request.token.clone(),
            location: payment_request.location.clone(),
        }).await?;

        Ok(web::Json(CreatePaymentRequestResponse {
            token: payment_request.token.clone()
        }))
    }

    #[post("/api/payment/swish/callback")] // Remember to change CALLBACK_URL
    pub async fn swish_callback(callback: web::Json<PaymentCallback>) -> ApiResult<()> {
        handle_callback(callback.into_inner()).await?;
        Ok(())
    }
}
