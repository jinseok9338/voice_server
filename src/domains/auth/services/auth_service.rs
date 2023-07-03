use diesel::pg::PgConnection;

use crate::domains::auth::dto::auth_dto::AuthResponse;

use super::database::auth_database::{create, make_token_invalid_by_user_id};

pub struct AuthService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> AuthService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn generate_token(&mut self, user_id: &i32) -> AuthResponse {
        let auth = create(self.conn, user_id);

        AuthResponse {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,

            expiration: auth.expiration.unwrap_or_else(|| {
                chrono::Utc::now().naive_utc() + chrono::Duration::seconds(3600)
            }),
        }
    }

    pub fn invalidate_token(&mut self, user_id: &i32) -> usize {
        make_token_invalid_by_user_id(self.conn, user_id)
    }
}
