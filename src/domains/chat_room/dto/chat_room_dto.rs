use std::{io::Write, str::FromStr};

use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
    AsChangeset, Insertable, Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{chat_rooms, sql_types::ChatTypeEnum};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = chat_rooms)]
pub struct ChatRoom {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub name: String,
    pub last_message: Option<String>,
    pub chat_type: TypeOfChat,
    pub id: Uuid,
    pub last_sent_user_id: Option<Uuid>,
}

impl ChatRoom {
    pub fn new(name: String, chat_type: TypeOfChat) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: chrono::offset::Utc::now().naive_utc(),
            updated_at: chrono::offset::Utc::now().naive_utc(),
            deleted_at: None,
            name,
            last_message: None,
            last_sent_user_id: None,
            chat_type,
        }
    }

    pub fn updated_chat_room(
        self: &Self,
        update_chat_room_request: &UpdateChatRoomRequest,
    ) -> Self {
        Self {
            id: self.id,
            created_at: self.created_at,
            updated_at: chrono::offset::Utc::now().naive_utc(),
            deleted_at: self.deleted_at,
            name: update_chat_room_request
                .name
                .clone()
                .unwrap_or(self.name.clone()),
            last_message: update_chat_room_request.last_message.clone(),
            last_sent_user_id: update_chat_room_request.last_sent_user_id.clone(),
            chat_type: self.chat_type,
        }
    }
}

pub struct UpdateChatRoomRequest {
    pub name: Option<String>,
    pub last_message: Option<String>,
    pub last_sent_user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChatRoomRequest {
    pub name: String,
    pub user_ids: Vec<Uuid>,
    pub type_of_chat: String,
}

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Deserialize, Serialize)]
#[sql_type = "ChatTypeEnum"]
pub enum TypeOfChat {
    Private,
    Group,
    Alone,
}

impl FromStr for TypeOfChat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRIVATE" => Ok(TypeOfChat::Private),
            "GROUP" => Ok(TypeOfChat::Group),
            "ALONE" => Ok(TypeOfChat::Alone),
            _ => Err(()),
        }
    }
}

impl ToSql<ChatTypeEnum, Pg> for TypeOfChat {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = match *self {
            TypeOfChat::Private => "PRIVATE",
            TypeOfChat::Group => "GROUP",
            TypeOfChat::Alone => "ALONE",
        };
        out.write_all(s.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<ChatTypeEnum, Pg> for TypeOfChat {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes()).map_err(|_| "Invalid UTF-8 string")?;
        match s {
            "PRIVATE" => Ok(TypeOfChat::Private),
            "GROUP" => Ok(TypeOfChat::Group),
            "ALONE" => Ok(TypeOfChat::Alone),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
