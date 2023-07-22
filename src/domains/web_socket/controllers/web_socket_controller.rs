use actix_http::HttpMessage;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{
    database::postgres_pool::Db,
    domains::{
        auth::services::jwt_service::Claims,
        chat_room::services::chat_room_service::ChatRoomService,
        web_socket::services::web_socket_service::MyWebSocket,
    },
};

#[get("ws/{chat_room_id}")]
pub async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let chat_room_id = req.match_info().get("chat_room_id").unwrap();
    let chat_room_id = chat_room_id.parse::<Uuid>().unwrap();
    let mut conn = Db::connect_to_db();
    let mut chat_room_service = ChatRoomService::new(&mut conn);
    let chat_room = chat_room_service.read_one_chat_rom(chat_room_id);
    match chat_room {
        Some(_) => {}
        None => return Ok(HttpResponse::NotFound().finish()),
    }
    let extenstions = req.extensions();
    let claims = extenstions.get::<Claims>();
    let user_id = match claims {
        Some(claims) => claims.user_id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    ws::start(MyWebSocket::new(user_id, chat_room_id), &req, stream)
}
