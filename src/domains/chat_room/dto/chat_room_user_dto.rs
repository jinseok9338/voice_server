// diesel::table! {
//     user_chat_room (user_id, chat_room_id) {
//         user_id -> Int4,
//         chat_room_id -> Int4,
//     }
// }

use diesel::{AsChangeset, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::schema::user_chat_room;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = user_chat_room)]
pub struct ChatRoomUserDatabase {
    pub user_id: i32,
    pub chat_room_id: i32,
}
