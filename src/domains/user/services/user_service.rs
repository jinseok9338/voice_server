
use diesel::pg::PgConnection;
use uuid::Uuid;

use crate::{
    domains::user::dto::user_dto::{User, UserWithOutPassword},
    errors::base_error_messages::BaseError,
};

use super::database::users::{
    _read, create, delete_one, read_one, read_one_user_by_name,
    read_one_user_by_user_id_with_password, update_last_login_at_to_database, update_one,
};

pub struct UserService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_user(&mut self, user: &User) -> User {
        create(self.conn, user)
    }

    pub fn _read_users(&mut self) -> Vec<User> {
        _read(self.conn)
    }

    pub fn read_one_user(&mut self, id: Uuid) -> Option<UserWithOutPassword> {
        read_one(self.conn, id)
    }

    pub fn reat_one_user_by_id(&mut self, id: Uuid) -> Option<User> {
        read_one_user_by_user_id_with_password(self.conn, id)
    }

    pub fn update_user(&mut self, id: Uuid, user: &User) -> UserWithOutPassword {
        update_one(self.conn, id, user)
    }

    pub fn update_last_login_at(&mut self, user_id: &Uuid) -> Result<(), BaseError> {
        // find the user and update the last_login_at
        update_last_login_at_to_database(self.conn, user_id);
        Ok(())
    }

    pub fn read_one_user_by_user_name(&mut self, user_name: &str) -> Option<User> {
        read_one_user_by_name(self.conn, user_name)
    }

    pub fn delete_user(&mut self, id: Uuid) -> usize {
        delete_one(self.conn, id)
    }
}
