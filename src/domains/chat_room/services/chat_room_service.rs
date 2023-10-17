use uuid::Uuid;

use crate::{
    consts::{Conn, DbPool},
    domains::chat_room::dto::chat_room_dto::ChatRoom,
};

use super::{
    chat_room_user_service::ChatRoomUserService,
    database::chat_room_database::{create, read_all_by_chat_room_ids, read_one, update},
};

pub struct ChatRoomService {
    pub conn: Conn,
}

impl ChatRoomService {
    pub fn new(pool: DbPool) -> Self {
        let conn = pool.get().expect("Error connecting to the database");
        Self { conn }
    }

    pub fn create_chat_room(&mut self, chat_room: &ChatRoom) -> ChatRoom {
        create(&mut self.conn, chat_room)
    }

    pub fn read_chat_rooms(
        &mut self,
        id: Uuid,
        chat_room_user_service: &mut ChatRoomUserService,
    ) -> Vec<ChatRoom> {
        let chat_room_ids = chat_room_user_service.find_all_chat_room_ids_by_user_id(id);
        let chat_rooms = read_all_by_chat_room_ids(&mut self.conn, chat_room_ids);

        chat_rooms
    }

    pub fn read_one_chat_rom(&mut self, id: Uuid) -> Option<ChatRoom> {
        read_one(&mut self.conn, id)
    }

    pub fn update_chat_room(&mut self, chat_room: &ChatRoom) -> ChatRoom {
        update(&mut self.conn, chat_room)
    }
}
