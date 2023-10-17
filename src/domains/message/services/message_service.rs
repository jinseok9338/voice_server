use uuid::Uuid;

use crate::{
    consts::{Conn, DbPool},
    domains::{
        chat_room::{
            dto::chat_room_dto::{ChatRoom, UpdateChatRoomRequest},
            services::chat_room_service::ChatRoomService,
        },
        message::dto::message_dto::{Message, NewMessage, Pagination},
    },
};

use super::database::message_database::{create_message_to_database, read_all_by_chat_room_ids};

pub struct MessageService {
    pub conn: Conn,
}

impl MessageService {
    pub fn new(pool: DbPool) -> Self {
        let conn = pool.get().expect("Error connecting to the database");
        Self { conn }
    }

    pub fn create_message(
        &mut self,
        chat_room_id: Uuid,
        new_message: &NewMessage,
        user_id: Uuid,
        chat_room_service: &mut ChatRoomService,
    ) -> Message {
        let message =
            create_message_to_database(&mut self.conn, chat_room_id, new_message, user_id);
        // update the chat room last message, last_send_by

        let chat_room = chat_room_service
            .read_one_chat_rom(chat_room_id)
            .expect("chat room not found");
        let update_chat_room_request = UpdateChatRoomRequest {
            last_message: Some(new_message.message.clone()),
            last_sent_user_id: Some(user_id),
            name: None,
        };
        let updated_chat_room = ChatRoom::updated_chat_room(&chat_room, &update_chat_room_request);
        chat_room_service.update_chat_room(&updated_chat_room);
        message
    }

    pub fn read_messages_by_chat_room_id(
        &mut self,
        chat_room_id: Uuid,
        page: i32,
        size: Option<i32>,
        sort_by: Option<(&str, &str)>,
    ) -> Pagination<Message> {
        let pagination =
            read_all_by_chat_room_ids(&mut self.conn, chat_room_id, page, size, sort_by);
        pagination
    }
}
