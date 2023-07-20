use std::env;

use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

use crate::schema::auths;
use dotenv::dotenv;
use uuid::Uuid;

#[derive(Queryable, QueryableByName, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = auths)]
pub struct Auth {
    #[column_name = "access_token"]
    pub access_token: String,
    #[column_name = "refresh_token"]
    pub refresh_token: String,
    #[column_name = "created_at"]
    pub created_at: Option<NaiveDateTime>,
    #[column_name = "is_valid"]
    pub is_valid: bool,
    #[column_name = "expiration"]
    pub expiration: Option<NaiveDateTime>,
    #[column_name = "auth_provider"]
    pub auth_provider: String,
    #[column_name = "user_id"]
    pub user_id: Option<Uuid>,
    #[column_name = "id"]
    pub id: Uuid,
}

impl Auth {
    pub fn new(
        access_token: String,
        refresh_token: String,
        user_id: Option<Uuid>,
        created_at: Option<NaiveDateTime>,
        is_valid: bool,
        auth_provider: String,
    ) -> Self {
        dotenv().ok();
        let expiration = env::var("ACCESS_TOKEN_EXPIRES_IN")
            .expect("EXPIRATION_TIME must be set")
            .parse::<i64>()
            .expect("EXPIRATION_TIME must be a valid integer");
        Self {
            id: Uuid::new_v4(),
            access_token,
            refresh_token,
            user_id,
            created_at,
            is_valid,
            expiration: Some(
                chrono::Utc::now().naive_utc() + chrono::Duration::seconds(expiration),
            ),
            auth_provider,
        }
    }
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
