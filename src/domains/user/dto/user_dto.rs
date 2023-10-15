use chrono::NaiveDateTime;
use diesel::sql_types::{Bool, Nullable, Text, Timestamptz, Uuid as dUuid};
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::users::{self};

#[derive(QueryableByName, Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(sql_type = Text)]
    pub username: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub password: Option<String>,
    #[diesel(sql_type = Text)]
    pub email: String,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub last_login_at: Option<NaiveDateTime>,
    #[diesel(sql_type = Nullable<Text>)]
    pub user_image: Option<String>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub updated_at: Option<NaiveDateTime>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub tester: Option<bool>,
    #[diesel(sql_type = dUuid)]
    pub id: Uuid,
}

#[derive(QueryableByName, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct UserWithOutPassword {
    pub username: String,
    pub email: String,
    pub last_login_at: Option<NaiveDateTime>,
    pub user_image: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub tester: Option<bool>,
    pub id: Uuid,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        email: String,
        user_image: Option<String>,
    ) -> Self {
        Self {
            username,
            password: Some(password),
            email,
            last_login_at: None,
            user_image,
            created_at: Some(chrono::offset::Utc::now().naive_utc()),
            updated_at: Some(chrono::offset::Utc::now().naive_utc()),
            tester: Some(false),
            id: Uuid::new_v4(),
        }
    }

    pub fn updated_user(
        existing_user: &User,
        username: Option<String>,
        email: Option<String>,
        last_login_at: Option<NaiveDateTime>,
        user_image: Option<String>,
    ) -> Self {
        Self {
            username: username.unwrap_or(existing_user.username.clone()),
            password: existing_user.password.clone(),
            email: email.unwrap_or(existing_user.email.clone()),
            last_login_at: last_login_at.or(existing_user.last_login_at),
            user_image,
            created_at: existing_user.created_at,
            updated_at: Some(chrono::offset::Utc::now().naive_utc()),
            tester: existing_user.tester,
            id: existing_user.id,
        }
    }

    pub fn user_without_password(user: &User) -> UserWithOutPassword {
        UserWithOutPassword {
            username: user.username.clone(),
            email: user.email.clone(),
            last_login_at: user.last_login_at,
            user_image: user.user_image.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            tester: user.tester,
            id: user.id,
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub tester: Option<bool>,
    pub user_image: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_image: Option<String>,
}
