use diesel::{AsChangeset, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user_chat_room;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = user_chat_room)]
pub struct ChatRoomUserDatabase {
    pub user_id: Uuid,
    pub chat_room_id: Uuid,
    pub id: Uuid,
}

pub struct ChatRoomId {
    pub chat_room_id: Uuid,
}
