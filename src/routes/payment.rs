//! Payment routes, currently only [`swish`] (Swish is the only supported [`PaymentMethod`]).

/// Supported payment methods. Currently unused elsewhere in the codebase; only
/// [`swish`] is implemented.
pub enum PaymentMethod {
    Swish,
}

/// Swish deposit flow: creating a payment request, receiving Swish's async
/// callback, polling status, and generating a QR code for the Swish app to scan.
pub mod swish {
    use actix_web::{HttpRequest, HttpResponse, get, http::StatusCode, post, web::{self, Data}};
    use uuid::Uuid;

    use crate::{AppState, Role, database::{self, crud, model::SwishPaymentRequestRow}, error::{ApiResult, AppError, ClientError, GenericError, SwishErrorResponse}, model::PendingTransaction, return_err, routes::{CurrentUser}};

    /// Path Swish POSTs payment status updates to. Must match [`swish_callback`]'s route attribute.
    pub const CALLBACK_URL: &str = "/api/payment/swish/callback"; // If changing URL: Remember to change post function
    /// Swish's API for generating a scannable QR code for a payment request token.
    pub const SWISH_QR_CODE_API: &str = "https://mpc.getswish.net/qrg-swish/api/v1/commerce";
    
    #[derive(serde::Serialize)]
    #[allow(non_snake_case)]
    /// Info we send to Swish to create a request
    pub struct PaymentRequestObject {
        payeeAlias: String,
        amount: f32,
        currency: String,
        callbackUrl: String,
        message: String,
        callbackIdentifier: String, // Sätt att skydda oss
    }

    impl PaymentRequestObject {
        /// Builds the request body for creating a Swish payment request of `amount`
        /// SEK, with a fresh random `callbackIdentifier` used later to authenticate
        /// Swish's callback (see [`swish_callback`]).
        pub fn new(state: &Data<AppState>, amount: f32) -> Self {
            PaymentRequestObject {
                payeeAlias: state.env.swish_number.clone(),
                amount,
                currency: String::from("SEK"),
                callbackUrl: String::from(state.env.site_domain.clone() + CALLBACK_URL),
                message: String::from("Konsfekt Betalning"),
                callbackIdentifier: Uuid::new_v4().to_string()
            }
        }
    }

    /// Body Swish POSTs to [`CALLBACK_URL`] when a payment request's status changes.
    #[derive(serde::Deserialize, Debug)]
    #[allow(non_snake_case, dead_code)]
    pub struct PaymentCallback {
        id: String,
        payeePaymentReference: Option<String>,
        paymentReference: Option<String>,
        callbackUrl: String,
        payerAlias: Option<String>,
        payeeAlias: String,
        amount: f64,
        currency: String,
        message: String,
        status: String,
        dateCreated: String,
        datePaid: Option<String>,
        errorCode: Option<String>,
        errorMessage: Option<String>,
    }

    /// Result of successfully creating a payment request with Swish, as parsed
    /// from its `201 Created` response.
    pub struct SwishPaymentResponse {
        payment_id: String,
        token: String,
        location: String,
        callback_identifier: String
    }

    /// Lifecycle status of a Swish payment request, mirrored from Swish's own status strings.
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, sqlx::Type)]
    #[serde(rename_all = "lowercase")]
    #[sqlx(type_name = "swish_status", rename_all = "lowercase")]
    pub enum Status {
        Pending,
        Paid,
        Declined,
        Error,
        Cancelled,
    }

    impl std::str::FromStr for Status {
        type Err = GenericError;
        /// Parses one of Swish's status strings (case-insensitive) into a [`Status`].
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "pending" => Ok(Status::Pending),
                "paid" => Ok(Status::Paid),
                "declined" => Ok(Status::Declined),
                "error" => Ok(Status::Error),
                "cancelled" => Ok(Status::Cancelled),
                unknown => Err(GenericError::new("Could not parse Swish payment status")
                    .with_status(StatusCode::BAD_REQUEST)
                    .add_info(format!("Found {unknown}")))
            }
        }
    }

    /// Creates a Swish payment request for `amount` SEK via Swish's `PUT`
    /// payment-requests endpoint, generating our own UUID as the payment id.
    async fn initiate_payment(state: &Data<AppState>, amount: f32) -> Result<SwishPaymentResponse, AppError> {
        // Our's and Swish's payment identifier
        let payment_id: String = Uuid::new_v4().simple().to_string().to_uppercase(); 
        let pro = PaymentRequestObject::new(state, amount);
        

        let response = state.client
            .put(format!("{}{}", state.env.swish_api_url, payment_id))
            .json(&pro)
            .send().await.map_err(ClientError::from)?;

        let status = response.status();

        if status == 201 {
            let token = response.headers().get("PaymentRequestToken").and_then(|t| t.to_str().ok());
            let location = response.headers().get("Location").and_then(|l| l.to_str().ok());
            return match (token, location) {
                (Some(token), Some(location)) => {
                    Ok(SwishPaymentResponse {
                        payment_id,
                        token: String::from(token),
                        location: String::from(location),
                        callback_identifier: pro.callbackIdentifier,
                    })
                },
                _ => Err(GenericError::new("Could not find/parse Swish response token or location").into())
            }
        }
        Err(SwishErrorResponse::to_error(response).await.into())
    }

    /// Query params for [`create_payment_request`].
    #[derive(serde::Deserialize)]
    struct CreatePaymentRequestQuery { amount: f32 }

    /// Response body for [`create_payment_request`].
    #[derive(serde::Serialize)]
    struct CreatePaymentRequestResponse {
        payment_id: String,
        token: String
    }

    /// `POST /api/payment/swish/create_payment_request?amount=<f32>` — starts a
    /// Swish deposit of at least 30 SEK for the current user: creates the request
    /// with Swish and records it as [`Status::Pending`]. The actual balance update
    /// happens later, in [`swish_callback`], once Swish confirms payment.
    /// Requires an authenticated user.
    #[post("/api/payment/swish/create_payment_request")]
    pub async fn create_payment_request(state: Data<AppState>, current_user: CurrentUser, query: web::Query<CreatePaymentRequestQuery>) -> ApiResult<web::Json<CreatePaymentRequestResponse>> {
        // let user = user_from_cookie(&state.db, &req).await?;
        let user = current_user.require_role(Role::User)?.into_row();

        if query.amount < 30.0 {
            return_err!(actix_web::error::ErrorBadRequest("amount < 30 kr"));
        }

        let swish_payment_response = initiate_payment(&state, query.amount).await?;
        let _ = crud::create_payment_request(&state.db, SwishPaymentRequestRow {
            id: swish_payment_response.payment_id.clone(),
            user: user.id,
            amount: query.amount,
            status: Status::Pending,
            token: swish_payment_response.token.clone(),
            callback_identifier: swish_payment_response.callback_identifier,
            location: swish_payment_response.location.clone(),
        }).await?;

        log::info!("User {} initiated a Swish payment", user.id);

        Ok(web::Json(CreatePaymentRequestResponse {
            payment_id: swish_payment_response.payment_id,
            token: swish_payment_response.token.clone()
        }))
    }

    /// `POST /api/payment/swish/callback` — Swish's async status-update webhook
    /// (URL registered with Swish as [`CALLBACK_URL`]; whitelisted in
    /// [`crate::routes::PATH_WHITELIST`] since Swish has no session cookie).
    ///
    /// Authenticates the callback via the `callbackIdentifier` header, matching it
    /// against the value stored when the payment request was created — anyone
    /// without it is rejected with `401`, so a POST here can't forge a payment. On
    /// a transition to [`Status::Paid`], credits the user's balance and records a
    /// deposit transaction (only once, guarded by comparing against the previously
    /// stored status).
    #[post("/api/payment/swish/callback")] // If changing URL: Remember to change CALLBACK_URL
    pub async fn swish_callback(state: Data<AppState>, req: HttpRequest, callback: web::Json<PaymentCallback>) -> ApiResult<()> {
        let payment_id = callback.id.clone();
        let payment_request = crud::get_payment_request(&state.db, payment_id.clone()).await?;
        let Some(callback_identifer) = req.headers().get("callbackIdentifier").and_then(|ci| ci.to_str().ok()) else {
            return_err!(actix_web::error::ErrorUnauthorized("Callback identifer not found"));
        };
        if callback_identifer != payment_request.callback_identifier {
            return_err!(actix_web::error::ErrorUnauthorized("Incorrect callback identifer"));
        }
        let payment_status = callback.status.parse::<Status>()?;
        crud::update_payment_request(&state.db, payment_id.clone(), payment_status).await?;
        
        if payment_request.status != payment_status { // If still Status::Pending
            let user = crud::get_user(&state.db, Some(payment_request.user), None).await?;
            log::info!("Updated user {}'s payment status to {:?} for payment {}", user.id, payment_status, payment_id);
            if payment_status == Status::Paid {
                crud::update_user_balance(&state.db, user.id, user.balance + payment_request.amount).await?;

                let transaction = PendingTransaction {
                    user: if user.private_transactions { None } else { Some(user.id) },
                    products: Vec::new(),
                    amount: payment_request.amount,
                    admin_issued: false
                };

                database::crud::create_transaction(&state.db, transaction).await?;
            }
        }

        Ok(())
    }

    /// Response body for [`check_status`].
    #[derive(serde::Serialize)]
    struct PaymentStatusResponse {
        status: Status,
        amount: f32,
        balance: f32,
    }

    /// `GET /api/payment/status/{payment_id}` — polls a Swish payment request's
    /// current status. Requires an authenticated user, and the payment request
    /// must belong to that user (`403` otherwise).
    #[get("/api/payment/status/{payment_id}")]
    pub async fn check_status(state: Data<AppState>, current_user: CurrentUser, path: web::Path<String>) -> ApiResult<web::Json<PaymentStatusResponse>> {
        // let user = user_from_cookie(&state.db, &req).await?;
        let user = current_user.require_role(Role::User)?.into_row();
        let payment_request = crud::get_payment_request(&state.db, path.into_inner()).await?;
        if payment_request.user != user.id {
            return_err!(actix_web::error::ErrorForbidden("Cannot get other user's payment status"));
        }
        Ok(web::Json(PaymentStatusResponse {
            status: payment_request.status,
            amount: payment_request.amount,
            balance: user.balance,
        }))
    }

    /// Request body sent to [`SWISH_QR_CODE_API`].
    #[derive(serde::Serialize)]
    struct QrCodeData {
        token: String,
        size: String,
        format: String,
        border: String,
    }

    /// `GET /api/payment/qr/{payment_id}` — fetches a 300x300 PNG QR code from
    /// Swish for the given payment request, for the Swish app to scan. Requires
    /// an authenticated user, and the payment request must belong to that user (`403` otherwise).
    #[get("/api/payment/qr/{payment_id}")]
    pub async fn get_qr_code(state: Data<AppState>, current_user: CurrentUser, path: web::Path<String>) -> ApiResult<HttpResponse> {
        // let user = user_from_cookie(&state.db, &req).await?;
        let user = current_user.require_role(Role::User)?.into_row();
        let payment_request = crud::get_payment_request(&state.db, path.into_inner()).await?;
        if payment_request.user != user.id {
            return_err!(actix_web::error::ErrorForbidden("Cannot retrieve QR code for other user's payment"));
        }
        let response = state.client.post(SWISH_QR_CODE_API)
            .json(&QrCodeData {
                token: payment_request.token, size: String::from("300"), 
                format: String::from("png"), border: String::from("0")
            }).send().await.map_err(ClientError::from)?;

        if response.status() != 200 {
            return_err!(actix_web::error::ErrorInternalServerError("Could not retrieve QR code from Swish"));
        }
        let qr_bytes = response.bytes().await.map_err(ClientError::from)?.to_vec();
        Ok(HttpResponse::Ok().content_type("image/png").body(qr_bytes))
    }
}
