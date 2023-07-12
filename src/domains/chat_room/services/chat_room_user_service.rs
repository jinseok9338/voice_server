use diesel::pg::PgConnection;



use crate::errors::base_error_messages::BaseError;

use super::database::chat_room_user_database::create_chat_rooms;

pub struct ChatRoomUserService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> ChatRoomUserService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_chat_rooms_user_relations(&mut self, user_ids: &Vec<i32>, chat_room_id:i32) -> Result<(),BaseError> {
        create_chat_rooms(self.conn, user_ids, chat_room_id)
    }

 
}