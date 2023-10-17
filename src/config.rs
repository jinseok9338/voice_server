use std::sync::Arc;

use actix_cors::Cors;
use actix_http::body::{BoxBody, EitherBody};
use actix_service::ServiceFactory;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    web::{self, ServiceConfig},
    App,
};
use tracing_actix_web::{StreamSpan, TracingLogger};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    domains::{
        auth::{controllers::auth_controller, dto::auth_dto::AuthResponse},
        chat_room::controllers::chat_room_controller,
        message::controllers::messages_controller,
        services::AppStateServices,
        user::{
            controllers::user_controllers,
            dto::user_dto::{AllUsers, NewUser, UserResponse},
        },
        web_socket::controllers::web_socket_controller,
    },
    middleware::custom_headers::CustomHeadersMiddleware,
};

#[derive(OpenApi)]
#[openapi(
    paths(auth_controller::signup, user_controllers::get_users),
    components(
        schemas(UserResponse, NewUser, AuthResponse),
        responses(AllUsers, AuthResponse)
    )
)]
struct ApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("auth")
            .service(auth_controller::login)
            .service(auth_controller::logout)
            .service(auth_controller::signup),
    );
}

pub fn setup_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}

pub fn setup_app(
    data: web::Data<Arc<AppStateServices>>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<StreamSpan<BoxBody>>>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app = App::new()
        .wrap(TracingLogger::default())
        .wrap(setup_cors())
        .wrap(CustomHeadersMiddleware)
        .app_data(data.clone());

    app.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
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
            .service(user_controllers::search_users)
            .service(user_controllers::get_users),
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
}
