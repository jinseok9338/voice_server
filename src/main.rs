mod database;
mod domains;
mod schema;

use actix_web::web::ServiceConfig;
use database::postgres_pool::Db;
use domains::cat::controllers::cat_controller;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(cat_controller::get_cats);
        cfg.service(cat_controller::create_cat);
        cfg.service(cat_controller::get_cat);
        cfg.service(cat_controller::update_cat);
        cfg.service(cat_controller::delete_cat);
    };

    Ok(config.into())
}
