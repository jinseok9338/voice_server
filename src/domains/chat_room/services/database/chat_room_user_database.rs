use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::domains::chat_room::dto::chat_room_user_dto::ChatRoomUserDatabase;
use crate::errors::base_error_messages::BaseError;
use crate::schema::user_chat_room;

pub fn create_chat_rooms(
    conn: &mut PgConnection,
    user_ids: &Vec<Uuid>,
    chat_room_id: Uuid,
) -> Result<(), BaseError> {
    // Implement your create logic here
    for user_id in user_ids {
        // Insert into user_chat_room table
        let new_user_chat_room = ChatRoomUserDatabase {
            user_id: *user_id,
            chat_room_id,
            id: Uuid::new_v4(),
        };
        diesel::insert_into(user_chat_room::table)
            .values(&new_user_chat_room)
            .execute(conn)
            .map_err(|err| BaseError::DatabaseError(err.into()))?;
    }
    Ok(())
}

pub fn get_all_chat_room_ids_by_user_id(conn: &mut PgConnection, user_id: Uuid) -> Vec<Uuid> {
    // find all chat room ids by user id in chat_room_user table
    let chat_room_ids = user_chat_room::table
        .filter(user_chat_room::user_id.eq(user_id))
        .select(user_chat_room::chat_room_id)
        .load::<Option<Uuid>>(conn)
        .expect("Error loading chat room ids");

    // convert Vec<Option<Uuid>> to Vec<Uuid>
    let mut chat_room_ids_vec = Vec::new();
    for chat_room_id in chat_room_ids {
        match chat_room_id {
            Some(id) => chat_room_ids_vec.push(id),
            None => (),
        }
    }
    chat_room_ids_vec
}
