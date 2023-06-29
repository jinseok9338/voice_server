use diesel::pg::PgConnection;

use crate::domains::cat::dto::{cat_dto::Cat, new_cat::NewCat};

pub struct CatService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> CatService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_cat(&mut self, cat: &NewCat) -> Cat {
        Cat::create(self.conn, cat)
    }

    pub fn read_cats(&mut self) -> Vec<Cat> {
        Cat::read(self.conn)
    }

    pub fn read_one_cat(&mut self, id: i32) -> Option<Cat> {
        Cat::read_one(self.conn, id)
    }

    pub fn update_cat(&mut self, cat: &Cat) -> Cat {
        cat.update(self.conn, cat)
    }

    pub fn delete_cat(&mut self, id: i32) -> usize {
        Cat::delete(self.conn, id)
    }
}
