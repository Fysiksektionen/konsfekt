use core::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};


/// Wraps the given error with a new type. From, Error & Display is implemented.
macro_rules! wrapper_error {
    ($name:ident($inner:ty)) => {
        #[derive(Debug)]
        pub struct $name(pub $inner);

        impl From<$inner> for $name {
            fn from(e: $inner) -> Self { Self(e) }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(&self.0)
            }
        }
    };
}

/// Implements [`ResponseError`] for a type with a given status code
macro_rules! impl_response_error {
    ($name:ident, $status:expr) => {
        impl ResponseError for $name {
            fn status_code(&self) -> actix_web::http::StatusCode {
                $status
            }
        
            fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
                if self.status_code().is_server_error() {
                    log::error!("{self}");
                } else {
                    log::debug!("{self}");
                }
                HttpResponse::build(self.status_code()).body(stringify!($name))
            }
        }
    };
}


wrapper_error!(DatabaseError(sqlx::Error));
wrapper_error!(ClientError(reqwest::Error));

impl_response_error!(DatabaseError, StatusCode::INTERNAL_SERVER_ERROR);
impl_response_error!(ClientError, StatusCode::INTERNAL_SERVER_ERROR);

#[derive(serde::Deserialize, Debug)]
pub struct SwishError {
    #[serde(rename = "errorCode")]
    code: String,
    #[serde(rename = "errorMessage")]
    message: String,
    #[serde(rename = "additionalInformation")]
    additional_info: String
}

impl fmt::Display for SwishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Swish error {}: {} Additional info: {}", self.code, self.message, self.additional_info)
    }
}

impl_response_error!(SwishError, StatusCode::INTERNAL_SERVER_ERROR);

impl SwishError {
    pub async fn from_response(resp: reqwest::Response) -> Option<SwishError> {
        match resp.json::<SwishError>().await {
            Ok(swish_error) => Some(swish_error),
            Err(_) => None
        }
    }
}


#[macro_export]
macro_rules! actix_err {
    ($error:expr) => {
        let error = $error;
        log::debug!("{}", error);
        return Err(error);
    };
}
