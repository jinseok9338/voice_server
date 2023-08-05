use uuid::Uuid;

use crate::domains::{
    chat_room::services::chat_room_user_service::ChatRoomUserService,
    notification::dto::notification_dto::{NotificationRequest, NotificationTypeEnum},
    user::services::user_service::UserService,
};

use super::postgres_pool::Db;

pub enum ProcessIncomingMessage {
    Chat {
        message: String,
        user_id: Uuid,
        users_to_notify: Vec<Uuid>,
    },
    // other variants will go here as you add them
}

pub fn process_incoming_message(message: ProcessIncomingMessage) -> String {
    match message {
        ProcessIncomingMessage::Chat {
            message,
            user_id,
            users_to_notify,
        } => {
            // users_to_notify make messages
            // and make notifications
            "1".to_string()
        }
    }
}

pub fn json_string_to_process_incoming_message_enum(json: &str) -> ProcessIncomingMessage {
    let notification_request: NotificationRequest = serde_json::from_str(json).unwrap();
    let user_id = notification_request.user_id;
    let users_to_notify = notification_request.users_to_notify;
    let notification_type = notification_request.notification_type;
    let data = notification_request.data;

    match notification_type {
        NotificationTypeEnum::Chat => ProcessIncomingMessage::Chat {
            message: data,
            user_id,
            users_to_notify,
        },
        // other variants will go here as you add them
    }
}

pub enum SendingRequestToRedis {
    Chat {
        message: String,
        user_id: Uuid,
        chat_room_id: Uuid,
    },
}

pub fn sending_request_to_redis(message: SendingRequestToRedis) -> String {
    match message {
        SendingRequestToRedis::Chat {
            message,
            user_id,
            chat_room_id,
        } => {
            // pg db connection
            let mut conn = Db::connect_to_db();
            let mut chat_room_user_service = ChatRoomUserService::new(&mut conn);
            let chat_room_users =
                chat_room_user_service.find_all_user_ids_with_chat_room_id(chat_room_id);

            let notification_request = NotificationRequest::new(
                user_id,
                chat_room_users,
                NotificationTypeEnum::Chat,
                message,
            );

            let notification_request_json = notification_request.to_json_string();

            notification_request_json
        }
    }
}
