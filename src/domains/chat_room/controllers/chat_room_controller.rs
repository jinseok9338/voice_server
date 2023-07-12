use actix_http::HttpMessage;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;

use crate::domains::chat_room::dto::chat_room_dto::{NewChatRoom, TypeOfChat};
use crate::domains::chat_room::services::chat_room_service::ChatRoomService;
use crate::domains::chat_room::services::chat_room_user_service::ChatRoomUserService;
use crate::Db;
use crate::{
    domains::chat_room::dto::chat_room_dto::NewChatRoomRequest,
    errors::base_error_messages::BaseError,
};

#[post("/chat_room")]
async fn create_chat_room(
    data: web::Json<NewChatRoomRequest>,
) -> Result<impl Responder, BaseError> {
    let mut conn = Db::connect_to_db();
    let now = chrono::offset::Utc::now().naive_utc();
    let new_chat_room = NewChatRoom {
        name: data.name.clone(),
        last_message: None,
        last_sent_user_id: None,
        created_at: now,
        updated_at: now,
        chat_type: data.type_of_chat.clone(),
    };

    let mut chat_room_service = ChatRoomService::new(&mut conn);
    let chat_room = chat_room_service.create_chat_room(&new_chat_room);
    let mut chat_room_user_service = ChatRoomUserService::new(&mut conn);

    let chat_room_user_relations =
        chat_room_user_service.create_chat_rooms_user_relations(&data.user_ids, chat_room.id);

    match chat_room_user_relations {
        Ok(_) => Ok(HttpResponse::Ok().json(chat_room)),
        Err(err) => {
            //revert the chat_room creation
            return Err(err);
        }
    }
}
