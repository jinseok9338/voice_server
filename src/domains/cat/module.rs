// this is where we combine the cat domain and export it to the main.rs
// src/routes/cat_routes.rs
use crate::domains::cat::controllers::cat_controller;
use actix_web::web::{self, ServiceConfig};

pub fn cat_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(cat_controller::get_cats)
            .service(cat_controller::create_cat)
            .service(cat_controller::get_cat)
            .service(cat_controller::update_cat)
            .service(cat_controller::delete_cat),
    );
}
