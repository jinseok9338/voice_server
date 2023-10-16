use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use super::auth::services::auth_service::AuthService;
use super::user::services::user_service::UserService;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppStateServices {
    pub pool: DbPool,
}

impl AppStateServices {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn user_service(&self) -> UserService {
        UserService::new(self.pool.clone())
    }

    pub fn auth_service(&self) -> AuthService {
        AuthService::new(self.pool.clone())
    }
}
