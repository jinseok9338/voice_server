use diesel::pg::PgConnection;
use uuid::Uuid;

use crate::domains::chat_room::dto::chat_room_dto::ChatRoom;

use super::{
    chat_room_user_service::ChatRoomUserService,
    database::chat_room_database::{create, read_all_by_chat_room_ids, read_one, update},
};

pub struct ChatRoomService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> ChatRoomService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_chat_room(&mut self, chat_room: &ChatRoom) -> ChatRoom {
        create(self.conn, chat_room)
    }

    pub fn read_chat_rooms(&mut self, id: Uuid) -> Vec<ChatRoom> {
        let mut chat_room_user_service = ChatRoomUserService::new(self.conn);
        let chat_room_ids = chat_room_user_service.find_all_chat_room_ids_by_user_id(id);
        let chat_rooms = read_all_by_chat_room_ids(self.conn, chat_room_ids);

        chat_rooms
    }

    pub fn read_one_chat_rom(&mut self, id: Uuid) -> Option<ChatRoom> {
        read_one(self.conn, id)
    }

    pub fn update_chat_room(&mut self, chat_room: &ChatRoom) -> ChatRoom {
        update(self.conn, chat_room)
    }

    // pub fn read_one_chat_room(&mut self, id: i32) -> Option<ChatRoom> {
    //     read_one(self.conn, id)
    // }

    // pub fn update_chat_room(&mut self, chat_room: &ChatRoom) -> ChatRoom {
    //     update_one(self.conn, chat_room.id, chat_room)
    // }

    // pub fn delete_chat_room(&mut self, id: i32) -> usize {
    //     delete_one(self.conn, id)
    // }
}
