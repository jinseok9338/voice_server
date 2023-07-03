use diesel::pg::PgConnection;

use crate::domains::user::dto::{new_user_dto::NewUser, user_dto::User};

use super::database::users::{
    _read, create, delete_one, read_one, read_one_user_by_name, update_one,
};

pub struct UserService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_user(&mut self, user: &NewUser) -> User {
        create(self.conn, user)
    }

    pub fn _read_users(&mut self) -> Vec<User> {
        _read(self.conn)
    }

    pub fn read_one_user(&mut self, id: i32) -> Option<User> {
        read_one(self.conn, id)
    }

    pub fn read_one_user_by_user_name(&mut self, user_name: &str) -> Option<User> {
        read_one_user_by_name(self.conn, user_name)
    }

    pub fn update_user(&mut self, id: i32, user: &NewUser) -> User {
        update_one(self.conn, id, user)
    }

    pub fn delete_user(&mut self, id: i32) -> usize {
        delete_one(self.conn, id)
    }
}
