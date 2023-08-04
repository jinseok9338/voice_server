use chrono::NaiveDateTime;
use diesel::sql_types::BigInt;

use diesel::{Insertable, Queryable, QueryableByName};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::messages;

#[derive(QueryableByName, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = messages)]
pub struct Message {
    pub message: String,
    pub sent_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub id: Uuid,
    pub chat_room_id: Option<Uuid>,
    pub sent_by: Option<Uuid>,
}

impl Message {
    pub fn new(chat_room_id: Uuid, message: String, sent_by: Uuid) -> Self {
        let now = chrono::offset::Utc::now().naive_utc();
        Self {
            message,
            sent_at: now,
            deleted_at: None,
            id: Uuid::new_v4(), // this should be uuid
            chat_room_id: Some(chat_room_id),
            sent_by: Some(sent_by),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMessage {
    pub message: String,
}

impl NewMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub content: Vec<T>,
    pub total_elements: i32,
    pub total_pages: i32,
    pub page: i32,
    pub size: i32,
    pub has_next: bool,
    pub has_previous: bool,
}

#[derive(QueryableByName)]
pub struct TotalElement {
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MessageQuery {
    pub page: i32,
    pub size: Option<i32>,
    pub sort_by: Option<String>,
}
