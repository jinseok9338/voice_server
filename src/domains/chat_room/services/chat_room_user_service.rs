use diesel::pg::PgConnection;
use uuid::Uuid;

use crate::errors::base_error_messages::BaseError;

use super::database::chat_room_user_database::{
    create_chat_rooms, find_all_user_ids_with_chat_room_id_from_database,
    get_all_chat_room_ids_by_user_id,
};

pub struct ChatRoomUserService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> ChatRoomUserService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_chat_rooms_user_relations(
        &mut self,
        user_ids: &Vec<Uuid>,
        chat_room_id: Uuid,
    ) -> Result<(), BaseError> {
        create_chat_rooms(self.conn, user_ids, chat_room_id)
    }

    pub fn find_all_chat_room_ids_by_user_id(&mut self, user_id: Uuid) -> Vec<Uuid> {
        get_all_chat_room_ids_by_user_id(self.conn, user_id)
    }

    pub fn match_the_user_with_chat_room(&mut self, user_id: Uuid, chat_room_id: Uuid) -> bool {
        let chat_room_ids = self.find_all_chat_room_ids_by_user_id(user_id);
        chat_room_ids.contains(&chat_room_id)
    }

    pub fn find_all_user_ids_with_chat_room_id(&mut self, chat_room_id: Uuid) -> Vec<Uuid> {
        let chat_room_ids =
            find_all_user_ids_with_chat_room_id_from_database(self.conn, chat_room_id);
        chat_room_ids
    }
}
