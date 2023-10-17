use std::sync::Arc;

use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::domains::{
    services::AppStateServices,
    web_socket::{dto::socket_dto::WebSocketQuery, services::web_socket_service::MyWebSocket},
};

#[get("ws/{chat_room_id}")]
pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    web::Query(query): web::Query<WebSocketQuery>,
    data: web::Data<Arc<AppStateServices>>,
) -> Result<HttpResponse, Error> {
    let chat_room_id = req.match_info().get("chat_room_id").unwrap();
    let chat_room_id = chat_room_id.parse::<Uuid>().unwrap();
    let data = data.into_inner();

    let mut chat_room_service = data.chat_room_service();
    let chat_room = chat_room_service.read_one_chat_rom(chat_room_id);

    match chat_room {
        Some(_) => {}
        None => return Ok(HttpResponse::NotFound().finish()),
    }

    let user_id = query.user_id;

    let mut chat_room_user_service = data.chat_room_user_service();
    let chat_room_exist =
        chat_room_user_service.match_the_user_with_chat_room(user_id, chat_room_id);

    if !chat_room_exist {
        return Ok(HttpResponse::NotFound().finish());
    }

    let chat_room_user_service = data.chat_room_user_service();
    let chat_room_service = data.chat_room_service();
    let message_service = data.message_service();
    ws::start(
        MyWebSocket::new(
            user_id,
            chat_room_id,
            chat_room_user_service,
            message_service,
            chat_room_service,
        ),
        &req,
        stream,
    )
}
