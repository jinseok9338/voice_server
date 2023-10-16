pub mod config;
pub mod database;
pub mod domains;
pub mod errors;
pub mod middleware;
pub mod schema;

use std::sync::Arc;

use crate::domains::chat_room::controllers::chat_room_controller;
use crate::domains::message::controllers::messages_controller;
use crate::domains::services::AppStateServices;

use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::*;

use crate::domains::web_socket::controllers::web_socket_controller;
use crate::domains::{auth::controllers::auth_controller, user::controllers::user_controllers};
use crate::middleware::auth_middleware::AuthMiddleware;
use crate::middleware::custom_headers::CustomHeadersMiddleware;
use actix_http::header;

use actix_web::{web, App};

use actix_cors::Cors;

use actix_web::HttpServer;

use database::postgres_pool::Db;

use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(auth_controller::signup))]
    struct ApiDoc;

    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    log::info!("starting HTTP server at http://localhost:8000");

    let pool = Db::create_db_pool();
    let app_services = Arc::new(AppStateServices::new(pool));
    let data = web::Data::new(app_services);

    HttpServer::new(move || {
        let app = App::new()
            .wrap(TracingLogger::default())
            .wrap(setup_cors())
            .wrap(CustomHeadersMiddleware)
            .app_data(data.clone());

        app.service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("api-docs/openapi.json", ApiDoc::openapi()),
        )
        .service(
            web::scope("auth")
                .service(auth_controller::login)
                .service(auth_controller::logout)
                .service(auth_controller::signup)
                .service(auth_controller::reissue_token),
        )
        .service(
            web::scope("users")
                .service(user_controllers::get_me)
                .service(user_controllers::update_me)
                .service(user_controllers::search_users),
        )
        .service(
            web::scope("chats")
                .service(chat_room_controller::create_chat_room)
                .service(chat_room_controller::get_chat_rooms),
        )
        .service(
            web::scope("messages")
                .service(messages_controller::get_chat_rooms_messages)
                .service(messages_controller::create_message),
        )
        .service(web_socket_controller::websocket)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn setup_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}
