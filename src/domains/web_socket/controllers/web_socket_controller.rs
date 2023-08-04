use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{
    database::postgres_pool::Db,
    domains::{
        chat_room::services::{
            chat_room_service::ChatRoomService, chat_room_user_service::ChatRoomUserService,
        },
        web_socket::{dto::socket_dto::WebSocketQuery, services::web_socket_service::MyWebSocket},
    },
};

#[get("ws/{chat_room_id}")]
pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    web::Query(query): web::Query<WebSocketQuery>,
) -> Result<HttpResponse, Error> {
    let chat_room_id = req.match_info().get("chat_room_id").unwrap();
    let chat_room_id = chat_room_id.parse::<Uuid>().unwrap();
    let mut conn = Db::connect_to_db();
    let mut chat_room_service = ChatRoomService::new(&mut conn);
    let chat_room = chat_room_service.read_one_chat_rom(chat_room_id);

    match chat_room {
        Some(_) => {}
        None => return Ok(HttpResponse::NotFound().finish()),
    }

    let user_id = query.user_id;

    let mut conn = Db::connect_to_db();
    let mut chat_room_user_service = ChatRoomUserService::new(&mut conn);
    let chat_room_exist =
        chat_room_user_service.match_the_user_with_chat_room(user_id, chat_room_id);

    if !chat_room_exist {
        return Ok(HttpResponse::NotFound().finish());
    }

    ws::start(MyWebSocket::new(user_id, chat_room_id), &req, stream)
}
