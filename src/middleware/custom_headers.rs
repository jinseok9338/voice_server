use std::pin::Pin;

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    Error,
};
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures::Future;

pub struct CustomHeadersMiddleware;

impl<S, Req> Transform<S, ServiceRequest> for CustomHeadersMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<Req>, Error = Error>,
    S::Future: 'static,
    Req: 'static,
{
    type Response = ServiceResponse<Req>;
    type Error = Error;
    type InitError = ();
    type Transform = CustomHeadersMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CustomHeadersMiddlewareService { service })
    }
}

pub struct CustomHeadersMiddlewareService<S> {
    service: S,
}

impl<S, Req> Service<ServiceRequest> for CustomHeadersMiddlewareService<S>
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

    fn call(self: &CustomHeadersMiddlewareService<S>, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::HeaderValue::from_static("*"),
            );
            Ok(res)
        })
    }
}
