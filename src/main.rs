pub mod config;
pub mod consts;
pub mod database;
pub mod domains;
pub mod errors;
pub mod middleware;
pub mod schema;

use std::sync::Arc;

use crate::config::setup_app;
use crate::domains::services::AppStateServices;

use env_logger::Env;

// use crate::middleware::auth_middleware::AuthMiddleware;

use actix_web::web;
use actix_web::HttpServer;
use database::postgres_pool::Db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    log::info!("starting HTTP server at http://localhost:8000");

    let pool = Db::create_db_pool();
    let app_services = Arc::new(AppStateServices::new(pool));
    let data: web::Data<Arc<AppStateServices>> = web::Data::new(app_services);

    HttpServer::new(move || {
        let app = setup_app(data.clone());
        app
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
