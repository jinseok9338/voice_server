use diesel::prelude::*;
use diesel::PgConnection;

use crate::domains::chat_room::dto::chat_room_user_dto::ChatRoomUserDatabase;
use crate::errors::base_error_messages::BaseError;
use crate::schema::user_chat_room;

pub fn create_chat_rooms(
    conn: &mut PgConnection,
    user_ids: &Vec<i32>,
    chat_room_id: i32,
) -> Result<(), BaseError> {
    // Implement your create logic here
    for user_id in user_ids {
        // Insert into user_chat_room table
        let new_user_chat_room = ChatRoomUserDatabase {
            user_id: *user_id,
            chat_room_id,
        };
        diesel::insert_into(user_chat_room::table)
            .values(&new_user_chat_room)
            .execute(conn)
            .map_err(|err| BaseError::DatabaseError(err.into()))?;
    }
    Ok(())
}
