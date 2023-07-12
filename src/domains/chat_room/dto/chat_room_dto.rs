use std::{io::Write, str::FromStr};

use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
    AsChangeset, Expression, Insertable, Queryable,
};
use serde::{Deserialize, Serialize};

use crate::schema::{chat_rooms, sql_types::ChatTypeEnum};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = chat_rooms)]
pub struct ChatRoom {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub name: String,
    pub last_message: Option<String>,
    pub last_sent_user_id: Option<i32>,
    pub chat_type: TypeOfChat,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = chat_rooms)]
pub struct NewChatRoom {
    pub name: String,
    pub last_message: Option<String>,
    pub last_sent_user_id: Option<i32>,
    pub chat_type: TypeOfChat,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChatRoomRequest {
    pub name: String,
    pub user_ids: Vec<i32>,
    pub type_of_chat: TypeOfChat,
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
