use diesel::pg::PgConnection;

use crate::domains::cat::dto::{cat_dto::Cat, new_cat::NewCat};

use super::database::cat_database::{create, delete_one, read, read_one, update_one};

pub struct CatService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> CatService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_cat(&mut self, cat: &NewCat) -> Cat {
        create(self.conn, cat)
    }

    pub fn read_cats(&mut self) -> Vec<Cat> {
        read(self.conn)
    }

    pub fn read_one_cat(&mut self, id: i32) -> Option<Cat> {
        read_one(self.conn, id)
    }

    pub fn update_cat(&mut self, cat: &Cat) -> Cat {
        update_one(self.conn, cat.id, cat)
    }

    pub fn delete_cat(&mut self, id: i32) -> usize {
        delete_one(self.conn, id)
    }
}
