use crate::domains::auth::controllers::auth_controller;
use crate::middleware::auth_middleware::AuthMiddleware;

use actix_web::middleware::Logger;
use actix_web::web::{self, ServiceConfig};

pub fn auth_routes_with_auth(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(AuthMiddleware)
            .service(auth_controller::logout),
    );
}

pub fn auth_routes_without_auth(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("").service(auth_controller::login).service(
            web::scope("")
                .service(auth_controller::logout)
                .wrap(AuthMiddleware)
                .wrap(Logger::new("%a %{User-Agent}i")),
        ),
    );
}
