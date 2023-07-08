use actix_web::web::{self, ServiceConfig};

use crate::domains::auth::controllers::auth_controller;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("auth")
            .service(auth_controller::login)
            .service(auth_controller::logout)
            .service(auth_controller::signup),
    );
}
