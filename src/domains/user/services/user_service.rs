use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use uuid::Uuid;

use crate::{
    domains::user::dto::user_dto::{User, UserWithOutPassword},
    errors::base_error_messages::BaseError,
};

use super::database::users::{
    _read, create, delete_one, read_one, read_one_user_by_name,
    read_one_user_by_user_id_with_password, search_users_in_db, update_last_login_at_to_database,
    update_one,
};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct UserService {
    pub conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl UserService {
    pub fn new(pool: DbPool) -> Self {
        let conn = pool.get().expect("Error connecting to the database");
        Self { conn }
    }

    pub fn create_user(&mut self, user: &User) -> User {
        create(&mut self.conn, user)
    }

    pub fn _read_users(&mut self) -> Vec<User> {
        _read(&mut self.conn)
    }

    pub fn search_users(&mut self, search_term: &str) -> Vec<UserWithOutPassword> {
        search_users_in_db(&mut self.conn, search_term)
    }

    pub fn read_one_user(&mut self, id: Uuid) -> Option<UserWithOutPassword> {
        read_one(&mut self.conn, id)
    }

    pub fn reat_one_user_by_id(&mut self, id: Uuid) -> Option<User> {
        read_one_user_by_user_id_with_password(&mut self.conn, id)
    }

    pub fn update_user(&mut self, id: Uuid, user: &User) -> UserWithOutPassword {
        update_one(&mut self.conn, id, user)
    }

    pub fn update_last_login_at(&mut self, user_id: &Uuid) -> Result<(), BaseError> {
        // find the user and update the last_login_at
        update_last_login_at_to_database(&mut self.conn, user_id);
        Ok(())
    }

    pub fn read_one_user_by_user_name(&mut self, user_name: &str) -> Option<User> {
        read_one_user_by_name(&mut self.conn, user_name)
    }

    pub fn delete_user(&mut self, id: Uuid) -> usize {
        delete_one(&mut self.conn, id)
    }
}
