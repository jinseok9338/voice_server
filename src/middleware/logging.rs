use actix_http::h1::Payload;
use actix_service::Service;
use actix_web::{
    dev::{self, ServiceRequest, ServiceResponse},
    error::Error,
    web,
};
use futures_util::future::LocalBoxFuture;
use log::error;
use serde_json::Value;
use std::rc::Rc;

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // extract bytes from request body
            let body = req.extract::<web::Bytes>().await.unwrap();

            let v: Value = serde_json::from_slice(&body)?;
            let pretty_body = serde_json::to_string_pretty(&v)?;

            println!("request path: {}", req.path());
            println!("request body: {}", pretty_body);
            println!("request headers: {:?}", req.headers());

            // re-insert body back into request to be used by handlers
            req.set_payload(bytes_to_payload(body));

            let res = svc.call(req).await;

            if let Err(e) = res {
                // Log the error message
                error!("An error occurred while processing the request: {}", e);
                return Err(e);
            }

            let res = res.unwrap();

            println!("response headers: {:?}", res.headers());

            Ok(res)
        })
    }
}

fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}
