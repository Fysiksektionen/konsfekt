use std::{future::{ready, Ready}, pin::Pin, rc::Rc};

use actix_web::{self, body::BoxBody, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, web, Error, HttpMessage, HttpResponse};

use crate::{auth::{self, validate_session}, AppState};

const AUTHORIZED_ROUTES: [&str; 2] = [
    "/auth/google",
    "/auth/google/callback",
];

const LOGIN_PATH: &str = "/login";

macro_rules! redirect_response {
    ($req:expr, $path:literal) => {
        $req.into_response(
            HttpResponse::Found()
                .append_header(("Location",  $path))
                .finish()
        )
    };
}

pub struct AuthRedirect;

impl<S> Transform<S, ServiceRequest> for AuthRedirect 
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthRedirectMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRedirectMiddleware { service: Rc::new(service) }))
    }
}

type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub struct AuthRedirectMiddleware<S> {
    service: Rc<S>
}

impl<S> Service<ServiceRequest> for AuthRedirectMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        
        if AUTHORIZED_ROUTES.contains(&req.path()) {
            return Box::pin(self.service.call(req));
        }

        let service = Rc::clone(&self.service);
        let is_login_path = req.path() == LOGIN_PATH;

        return Box::pin(async move {
            let state = req.app_data::<web::Data<AppState>>().unwrap();

            match auth::parse_auth_cookie(req.cookie(auth::AUTH_COOKIE)) {
                
                Some(token) => match validate_session(&state.db, token).await {

                    // TODO: Vad ska hända när dessa stöts på? Redirect till login, eller error?
                    Ok(None) => Err(actix_web::error::ErrorUnauthorized("Session token unauthorized")),
                    Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
                    
                    // Good Validation
                    Ok(session) => {
                        req.extensions_mut().insert(session);

                        if is_login_path {
                            return Ok(redirect_response!(req, "/"));
                        }

                        service.call(req).await
                    }
                }

                // No cookie
                None => if is_login_path { service.call(req).await } 
                        else { Ok(redirect_response!(req, "/login")) }
            }
        });
    }
}
