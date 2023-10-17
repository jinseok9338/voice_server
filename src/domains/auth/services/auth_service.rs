use uuid::Uuid;

use crate::{
    consts::{Conn, DbPool},
    domains::auth::dto::auth_dto::{Auth, AuthResponse},
};

use super::database::auth_database::{
    create, get_auth_by_refresh_token_from_data_base, get_auth_by_token,
    make_token_invalid_by_user_id,
};

pub struct AuthService {
    pub conn: Conn,
}

impl AuthService {
    pub fn new(pool: DbPool) -> Self {
        let conn = pool.get().expect("Error connecting to the database");
        Self { conn }
    }

    pub fn generate_token(&mut self, user_id: &Uuid) -> AuthResponse {
        let auth = create(&mut self.conn, user_id);

        AuthResponse {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,

            expiration: auth.expiration.unwrap_or_else(|| {
                chrono::Utc::now().naive_utc() + chrono::Duration::seconds(3600)
            }),
        }
    }

    pub fn invalidate_token(&mut self, user_id: &Uuid) -> usize {
        make_token_invalid_by_user_id(&mut self.conn, user_id)
    }

    pub fn get_auth_by_access_token(&mut self, token: &str) -> Option<Auth> {
        get_auth_by_token(&mut self.conn, token)
    }

    pub fn get_auth_by_refresh_token(&mut self, token: &str) -> Option<Auth> {
        get_auth_by_refresh_token_from_data_base(&mut self.conn, token)
    }
}
