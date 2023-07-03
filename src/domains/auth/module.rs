use super::controllers::auth_controller;
use actix_web::web::{self, ServiceConfig};

pub fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(auth_controller::signup)
            .service(auth_controller::login)
            .service(auth_controller::logout),
    );
}
