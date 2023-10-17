use diesel::prelude::*;

use uuid::Uuid;

use crate::consts::Conn;
use crate::domains::chat_room::dto::chat_room_dto::ChatRoom;

use crate::schema::chat_rooms;

pub fn create(conn: &mut Conn, chat_room: &ChatRoom) -> ChatRoom {
    diesel::insert_into(chat_rooms::table)
        .values(chat_room)
        .get_result(&mut *conn)
        .expect("Error saving new chat room")
}

pub fn read_all_by_chat_room_ids(conn: &mut Conn, chat_room_ids: Vec<Uuid>) -> Vec<ChatRoom> {
    chat_rooms::table
        .filter(chat_rooms::id.eq_any(chat_room_ids))
        .load::<ChatRoom>(&mut *conn)
        .expect("Error loading chat rooms")
}

pub fn read_one(conn: &mut Conn, id: Uuid) -> Option<ChatRoom> {
    chat_rooms::table
        .find(id)
        .first(&mut *conn)
        .optional()
        .expect("Error loading chat room")
}

pub fn update(conn: &mut Conn, chat_room: &ChatRoom) -> ChatRoom {
    diesel::update(chat_rooms::table)
        .filter(chat_rooms::id.eq(chat_room.id))
        .set(chat_room)
        .get_result(&mut *conn)
        .expect("Error updating chat room")
}
