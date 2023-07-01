use diesel::prelude::*;
use diesel::PgConnection;

use crate::domains::cat::dto::cat_dto::Cat;
use crate::domains::cat::dto::new_cat::NewCat;
use crate::schema::cats;

pub fn create(conn: &mut PgConnection, cat: &NewCat) -> Cat {
    diesel::insert_into(cats::table)
        .values(cat)
        .get_result(conn)
        .expect("Error saving new cat")
}

pub fn read(conn: &mut PgConnection) -> Vec<Cat> {
    cats::table
        .order(cats::id.desc())
        .load::<Cat>(conn)
        .expect("Error reading cats")
}

pub fn read_one(conn: &mut PgConnection, id: i32) -> Option<Cat> {
    cats::table.find(id).first(conn).ok()
}

pub fn update_one(conn: &mut PgConnection, id: i32, new_cat: &Cat) -> Cat {
    diesel::update(cats::table.find(id))
        .set(new_cat)
        .get_result(conn)
        .expect("Error updating cat")
}

pub fn delete_one(conn: &mut PgConnection, id: i32) -> usize {
    diesel::delete(cats::table.find(id))
        .execute(conn)
        .expect("Error deleting cat")
}
