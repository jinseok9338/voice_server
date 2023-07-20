use actix_http::HttpMessage;
use actix_web::{get, post, web, HttpRequest, Responder};

use uuid::Uuid;

use crate::database::postgres_pool::Db;
use crate::domains::auth::services::jwt_service::Claims;
use crate::domains::message::dto::message_dto::{MessageQuery, NewMessage};

use crate::domains::message::services::message_service::MessageService;
use crate::errors::base_error_messages::{BaseError, BaseErrorMessages};

#[get("/{chat_room_id}")]
async fn get_chat_rooms_messages(
    path: web::Path<String>,
    params: web::Query<MessageQuery>,
) -> Result<impl Responder, BaseError> {
    // get the chat_room_id from the url
    let chat_room_id = path.into_inner();
    let chat_room_id = chat_room_id
        .parse::<Uuid>()
        .expect("chat_room_id is not a Uuid");
    // get all the messages from the message service by chat_room_id
    let page = params.page;
    let size = params.size;
    let sort_by = &params.sort_by;
    // split the sort_by into a tuple, split it with a comma
    let sort_by = match sort_by {
        Some(sort_by) => {
            let sort_by: Vec<&str> = sort_by.split(",").collect();
            Some((sort_by[0], sort_by[1]))
        }
        None => None,
    };
    let mut conn = Db::connect_to_db();
    let mut message_service = MessageService::new(&mut conn);
    let message_pagination =
        message_service.read_messages_by_chat_room_id(chat_room_id, page, size, sort_by);
    Ok(web::Json(message_pagination))
}

#[post("/{chat_room_id}")]
async fn create_message(
    path: web::Path<String>,
    message: web::Json<NewMessage>,
    req: HttpRequest,
) -> Result<impl Responder, BaseError> {
    // get the chat_room_id from the url
    let claims = req.extensions();
    let claims = claims.get::<Claims>();

    let claims = match claims {
        Some(claims) => claims,
        None => {
            return Err(BaseError::NotFound(BaseErrorMessages::new(
                "User not found".to_string(),
                1,
            )))
        }
    };

    let user_id = claims.user_id;
    let chat_room_id = path.into_inner();
    let chat_room_id = chat_room_id
        .parse::<Uuid>()
        .expect("chat_room_id is not a Uuid");
    // get the message from the body
    let message = message.into_inner();
    // create the message
    let mut conn = Db::connect_to_db();
    let mut message_service = MessageService::new(&mut conn);
    let message = message_service.create_message(chat_room_id, &message, user_id);
    Ok(web::Json(message))
}
