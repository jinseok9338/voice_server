use diesel::pg::PgConnection;

use crate::domains::chat_room::dto::chat_room_dto::{ChatRoom, NewChatRoom};

use super::database::chat_room_database::create;

pub struct ChatRoomService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> ChatRoomService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_chat_room(&mut self, chat_room: &NewChatRoom) -> ChatRoom {

        create(self.conn, chat_room)
    }

    // pub fn read_chat_rooms(&mut self) -> Vec<ChatRoom> {
    //     read(self.conn)
    // }

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
