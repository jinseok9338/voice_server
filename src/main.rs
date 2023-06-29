mod database;
mod domains;
mod schema;

use actix_web::web::ServiceConfig;
use database::postgres_pool::Db;
use domains::cat::module::cat_routes;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cat_routes(cfg);
    };

    Ok(config.into())
}
