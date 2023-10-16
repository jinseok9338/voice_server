use uuid::Uuid;

use crate::domains::{
    chat_room::services::chat_room_user_service::ChatRoomUserService,
    message::{dto::message_dto::NewMessage, services::message_service::MessageService},
    notification::{
        dto::notification_dto::{NotificationRequest, NotificationTypeEnum},
        services::notification_service::NotificationService,
    },
};

use super::postgres_pool::Db;

pub enum ProcessIncomingMessage {
    Chat {
        message: String,
        user_id: Uuid,
        users_to_notify: Vec<Uuid>,
        chat_room_id: Uuid,
    },
    // other variants will go here as you add them
}

pub fn process_incoming_message(message: ProcessIncomingMessage) -> String {
    match message {
        ProcessIncomingMessage::Chat {
            message,
            user_id,
            users_to_notify,
            chat_room_id,
        } => {
            // make message with user_id and chat_room_id and message
            let mut conn = Db::connect_to_db();
            let mut message_service = MessageService::new(&mut conn);
            let message = NewMessage::new(message);
            let message = message_service.create_message(chat_room_id, &message, user_id);
            log::debug!("message: {:?}", message.id);
            let mut conn = Db::connect_to_db();
            let mut notification_service = NotificationService::new(&mut conn);
            let notifications = notification_service.create_notifications(
                user_id,
                users_to_notify,
                NotificationTypeEnum::Chat,
                &message.message,
            );
            log::debug!("notification: {:?}", notifications.len());
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
    let chat_room_id = notification_request.chat_room_id;

    match notification_type {
        NotificationTypeEnum::Chat => ProcessIncomingMessage::Chat {
            message: data,
            user_id,
            users_to_notify,
            chat_room_id,
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
                chat_room_id,
            );

            let notification_request_json = notification_request.to_json_string();

            notification_request_json
        }
    }
}
