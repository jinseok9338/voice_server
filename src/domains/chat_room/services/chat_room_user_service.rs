use diesel::pg::PgConnection;
use uuid::Uuid;

use crate::errors::base_error_messages::BaseError;

use super::database::chat_room_user_database::{
    create_chat_rooms, get_all_chat_room_ids_by_user_id,
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
}
