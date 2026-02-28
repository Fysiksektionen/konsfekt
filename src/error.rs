use core::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};

/// Helper macro that logs and returns an [`actix_web::Error`]
#[macro_export]
macro_rules! actix_err {
    ($error:expr) => {
        let error = $error;
        log::debug!("{}", error);
        return Err(error);
    };
}

/// Wraps `$inner` in a newtype which implements [`std::fmt::Display`], [`From<$inner>`] and 
/// [`ResponseError`]. The newtype wrapper also becomes and error type that optionally can
/// implement [`std::error::Error::source`]. Pass `no_source` if the wrapped `$inner` type is not an error.
///
/// Example:
/// ```rust, ignore
/// app_error_enum! {
/// //  EnumVariant(WrapperError(InnnerType)),
///     Database(DatabaseError(sqlx::Error)),
///     Client(ClientError(reqwest::Error)),
///     Session(SessionError(String, no_source)),
/// }
///
/// ```
macro_rules! app_error_enum {
    ($($variant:ident($wrapper:ident($inner:ty $(, $no_source:ident)?))),+ $(,)?) => {
        $(
            #[derive(Debug)]
            pub struct $wrapper {
                pub inner: $inner,
                http_code: StatusCode,
            }

            impl $wrapper {
                pub fn new(inner: $inner) -> Self {
                    Self {
                        inner, http_code: StatusCode::INTERNAL_SERVER_ERROR
                    }
                }

                pub fn with_status(mut self, code: StatusCode) -> Self {
                    self.http_code = code;
                    self
                }
            }

            impl From<$inner> for $wrapper {
                fn from(e: $inner) -> Self { Self::new(e) }
            }

            impl std::fmt::Display for $wrapper {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.inner)
                }
            }

            impl From<$wrapper> for AppError {
                fn from(err: $wrapper) -> Self {
                    AppError::$variant(err)
                }
            }

            impl std::error::Error for $wrapper {
                app_error_enum!(@source $($no_source)?);
            }

            impl ResponseError for $wrapper {
                fn status_code(&self) -> actix_web::http::StatusCode {
                    self.http_code
                    
                }
            
                fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
                    HttpResponse::build(self.status_code()).body(stringify!($wrapper))
                }
            }
        )+

        #[derive(Debug)]
        pub enum AppError {
            $($variant($wrapper),)+
        }

    };
    (@source) => {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.inner)
        }
    };
    (@source no_source) => {
    };
}

#[derive(serde::Deserialize, Debug)]
pub struct SwishErrorResponse {
    #[serde(rename = "errorCode")]
    code: String,
    #[serde(rename = "errorMessage")]
    message: String,
    #[serde(rename = "additionalInformation")]
    additional_info: String,
    /// Underlying client HTTP code
    #[serde(skip_deserializing)]
    http_status: Option<reqwest::StatusCode>
}

impl fmt::Display for SwishErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Swish error {}: {} Additional info: {}", self.code, self.message, self.additional_info)
    }
}

impl SwishErrorResponse {
    pub async fn from_response(resp: reqwest::Response) -> Option<Self> {
        let status = resp.status();
        match resp.json::<SwishErrorResponse>().await {
            Ok(mut swish_error) => {
                swish_error.http_status = Some(status);
                Some(swish_error)
            },
            Err(_) => None
        }
    }
}

app_error_enum! {
    Database(DatabaseError(sqlx::Error)),
    Client(ClientError(reqwest::Error)),
    Auth(AuthError(&'static str, no_source)),
    Swish(SwishError(SwishErrorResponse, no_source)),
}

macro_rules! match_error_variant {
    ($match:ident, $on_match:expr) => {
        match $match {
            AppError::Database(err) => $on_match(err),
            AppError::Client(err) => $on_match(err),
            AppError::Auth(err) => $on_match(err),
            AppError::Swish(err) => $on_match(err),
        }
    };
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match_error_variant!(self, std::error::Error::source);
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match_error_variant!(self, |err| write!(f, "{err}"))
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        return match_error_variant!(self, ResponseError::status_code);
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if self.status_code().is_server_error() {
            log::error!("{self}");
        } else {
            log::debug!("{self}");
        }
        return match_error_variant!(self, ResponseError::error_response);
    }
}
