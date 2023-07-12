use diesel::prelude::*;
use diesel::PgConnection;

use crate::domains::chat_room::dto::chat_room_dto::ChatRoom;
use crate::domains::chat_room::dto::chat_room_dto::NewChatRoom;
use crate::schema::chat_rooms;

pub fn create(conn: &mut PgConnection, chat_room: &NewChatRoom) -> ChatRoom {
    // Implement your create logic here
    diesel::insert_into(chat_rooms::table)
        .values(chat_room)
        .get_result(conn)
        .expect("Error saving new chat room")
}
