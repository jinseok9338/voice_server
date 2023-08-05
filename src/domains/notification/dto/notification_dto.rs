use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use std::{io::Write, str::FromStr};
use uuid::Uuid;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
    Insertable, Queryable,
};

use crate::schema::{notifications, sql_types::NotificationType};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = notifications)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_to_notify: Option<Uuid>,
    pub notification_type: Option<NotificationTypeEnum>,
    pub data: Option<String>,
    pub read: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// make new impl for Notification

impl Notification {
    pub fn new(
        user_id: Uuid,
        user_to_notify: Uuid,
        notification_type: NotificationTypeEnum,
        data: String,
    ) -> Self {
        let now = chrono::offset::Utc::now().naive_utc();

        let now = now + chrono::Duration::hours(9);
        Self {
            id: Uuid::new_v4(),
            user_id: Some(user_id),
            user_to_notify: Some(user_to_notify),
            notification_type: Some(notification_type),
            data: Some(data),
            read: Some(false),
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NotificationRequest {
    pub user_id: Uuid,
    pub users_to_notify: Vec<Uuid>,
    pub notification_type: NotificationTypeEnum,
    pub data: String,
}

impl NotificationRequest {
    pub fn new(
        user_id: Uuid,
        users_to_notify: Vec<Uuid>,
        notification_type: NotificationTypeEnum,
        data: String,
    ) -> Self {
        Self {
            user_id,
            users_to_notify,
            notification_type,
            data,
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// make eunm for notification_type

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Deserialize, Serialize)]
#[diesel(sql_type = NotificationType)]
pub enum NotificationTypeEnum {
    Chat,
    // other variants will go here as you add them
}

impl FromStr for NotificationTypeEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHAT" => Ok(NotificationTypeEnum::Chat),
            // other variants will go here as you add them
            _ => Err(()),
        }
    }
}

impl ToSql<NotificationType, Pg> for NotificationTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = match *self {
            NotificationTypeEnum::Chat => "CHAT",
            // other variants will go here as you add them
        };
        out.write_all(s.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<NotificationType, Pg> for NotificationTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes()).map_err(|_| "Invalid UTF-8 string")?;
        match s {
            "CHAT" => Ok(NotificationTypeEnum::Chat),
            // other variants will go here as you add them
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
