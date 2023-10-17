use std::str::FromStr;
use std::sync::Arc;

use actix_http::HttpMessage;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::domains::auth::services::jwt_service::Claims;
use crate::domains::chat_room::dto::chat_room_dto::{ChatRoom, TypeOfChat};

use crate::domains::services::AppStateServices;
use crate::errors::base_error_messages::BaseErrorMessages;

use crate::{
    domains::chat_room::dto::chat_room_dto::NewChatRoomRequest,
    errors::base_error_messages::BaseError,
};

#[get("/chat_room")]
async fn get_chat_rooms(
    req: HttpRequest,
    data: web::Data<Arc<AppStateServices>>,
) -> Result<impl Responder, BaseError> {
    let data = data.into_inner();
    let extenstions = req.extensions();
    let claims = extenstions.get::<Claims>();
    let user_id = match claims {
        Some(claims) => claims.user_id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let mut chat_room_service = data.chat_room_service();
    let mut chat_room_user_service = data.chat_room_user_service();
    let chat_rooms = chat_room_service.read_chat_rooms(user_id, &mut chat_room_user_service);
    Ok(HttpResponse::Ok().json(chat_rooms))
}

#[post("/chat_room")]
async fn create_chat_room(
    data: web::Json<NewChatRoomRequest>,
    services: web::Data<Arc<AppStateServices>>,
) -> Result<impl Responder, BaseError> {
    let services = services.into_inner();
    let chat_room_type = match TypeOfChat::from_str(&data.type_of_chat) {
        Ok(t) => t,
        Err(_) => {
            return Err(BaseError::BadRequest(BaseErrorMessages::new(
                "Invalid chat type".to_string(),
                1,
            )))
        }
    };
    let new_chat_room = ChatRoom::new(data.name.clone(), chat_room_type);

    let mut chat_room_service = services.chat_room_service();
    let chat_room = chat_room_service.create_chat_room(&new_chat_room);
    let mut chat_room_user_service = services.chat_room_user_service();

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
