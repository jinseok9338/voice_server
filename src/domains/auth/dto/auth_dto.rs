use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::auths;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = auths)]
pub struct Auth {
    // Add fields here
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: Option<NaiveDateTime>,
    pub is_valid: bool,
    pub expiration: Option<NaiveDateTime>,
    pub auth_provider: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = auths)]
pub struct NewAuth {
    pub user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub is_valid: bool,
    pub expiration: Option<NaiveDateTime>,
    pub auth_provider: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expiration: NaiveDateTime,
}
#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}
