use diesel::pg::PgConnection;

use crate::domains::user::dto::{new_user_dto::NewUser, user_dto::User};

use super::database::users::{create, read, read_one, update_one, delete_one};



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

    pub fn read_users(&mut self) -> Vec<User> {
        read(self.conn)
    }

    pub fn read_one_user(&mut self, id: i32) -> Option<User> {
        read_one(self.conn, id)
    }

    pub fn update_user(&mut self, user: &User) -> User {
        update_one(self.conn, user.id, user)
    }

    pub fn delete_user(&mut self, id: i32) -> usize {
        delete_one(self.conn, id)
    }
}
