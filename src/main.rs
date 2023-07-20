pub mod config;
pub mod database;
pub mod domains;
pub mod errors;
pub mod middleware;
pub mod schema;

use crate::database::redis::create_redis_client;

use crate::domains::chat_room::controllers::chat_room_controller;
use crate::domains::message::controllers::messages_controller;
use crate::domains::{auth::controllers::auth_controller, user::controllers::user_controllers};
use crate::middleware::auth_middleware::AuthMiddleware;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use database::postgres_pool::Db;
use env_logger::Env;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("trace"));
    log::info!("starting HTTP server at http://localhost:8080");
    let redis_client = create_redis_client()
        .await
        .expect("Failed to create Redis client");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .wrap(AuthMiddleware)
            .app_data(web::Data::new(redis_client.clone())) // Pass Redis client as shared state
            .service(
                web::scope("auth")
                    .service(auth_controller::login)
                    .service(auth_controller::logout)
                    .service(auth_controller::signup),
            )
            .service(
                web::scope("users")
                    .service(user_controllers::get_me)
                    .service(user_controllers::update_me),
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
