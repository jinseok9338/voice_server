mod domains;
mod database;
mod schema;

use actix_web::{get, web::ServiceConfig};
use database::postgres_pool::Db;
use domains::cat::controllers::cat_controller;
use shuttle_actix_web::ShuttleActixWeb;


#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}


#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(cat_controller::get_cats);
        cfg.service(cat_controller::create_cat);
        cfg.service(cat_controller::get_cat);
        cfg.service(cat_controller::update_cat);
        cfg.service(cat_controller::delete_cat);
    };


    Ok(config.into())
}
