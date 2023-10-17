use actix_http::header::HeaderValue;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};

use actix_web::{Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures::Future;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::error;
use regex::Regex;
use std::pin::Pin;
use std::sync::Arc;

use crate::consts::AUTH_MIDDLEWARE_CHECK_PATHS;
use crate::domains::auth::services::auth_service::AuthService;
use crate::domains::auth::services::jwt_service::Claims;
use crate::domains::services::AppStateServices;
use crate::errors::base_error_messages::BaseError;

pub struct AuthMiddleware<S> {
    pub service: S,
    pub app_services: Arc<AppStateServices>,
}

impl<S, Req> Transform<S, ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<Req>, Error = Error>,
    S::Future: 'static,
    Req: 'static,
{
    type Response = ServiceResponse<Req>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service,
            app_services: self.app_services.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    app_services: Arc<AppStateServices>,
}

impl<S, Req> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<Req>, Error = Error>,
    S::Future: 'static,
    Req: 'static,
{
    type Response = ServiceResponse<Req>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(self: &Self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(self: &Self, req: ServiceRequest) -> Self::Future {
        log::debug!(
            "Request path: {}, method: {} ",
            req.path(),
            req.method().as_str(),
        );

        if req.method().as_str() == "OPTIONS" {
            // Skip the authentication check
            let fut = self.service.call(req);

            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        if let Some(excluded) = AUTH_MIDDLEWARE_CHECK_PATHS.iter().find(|entry| {
            let re = Regex::new(entry.path).unwrap();
            re.is_match(req.path()) && entry.method == req.method().as_str()
        }) {
            // If the request path and method are in the list of excluded paths, check the skip_check flag
            if excluded.skip_check {
                // Skip the authentication check
                let fut = self.service.call(req);

                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }
        // Perform the authentication check
        let headers_value: Option<&HeaderValue> = req.headers().get("Authorization");
        let access_token: &actix_http::header::HeaderValue = match headers_value {
            Some(req) => req,
            None => {
                error!(
                    "Unauthorized: There is no Access Token (file: {}, line: {})",
                    file!(),
                    line!()
                );
                return Box::pin(async move { Err(BaseError::Unauthorized.into()) });
            }
        };
        let mut auth_service = self.app_services.auth_service();
        let claims = match is_authenticated(access_token, &mut auth_service) {
            Some(claims) => claims,
            None => {
                error!("Unauthorized: There is no Claim associated with access token (file: {}, line: {})", file!(), line!());
                return Box::pin(async move { Err(BaseError::Unauthorized.into()) });
            }
        };

        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

fn is_authenticated(access_token: &HeaderValue, auth_service: &mut AuthService) -> Option<Claims> {
    dotenv::dotenv().ok();
    let token = access_token.to_str().unwrap().to_string();
    let token = token.replace("Bearer ", "");
    let token = token.trim();

    //check the token in auths table and see if it is valid first

    let auth = auth_service.get_auth_by_access_token(token);
    if auth.is_none() {
        error!(
            "Unauthorized: There is no Auth associated with access token (file: {}, line: {})",
            file!(),
            line!()
        );
        return None;
    }
    let secret = std::env::var("ACCESS_TOKEN_SECRET").unwrap();
    let validation = Validation::default();
    let key = DecodingKey::from_secret(secret.as_bytes());
    match decode::<Claims>(token, &key, &validation) {
        Ok(claims) => Some(claims.claims),
        Err(_) => None,
    }
}
