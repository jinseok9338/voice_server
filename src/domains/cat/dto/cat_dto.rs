use diesel::prelude::*;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::cats;

use super::new_cat::NewCat;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = cats)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub breed: String,
    pub color: String,
    pub weight: f64,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Cat {
    pub fn create(conn: &mut PgConnection, cat: &NewCat) -> Self {
        diesel::insert_into(cats::table)
            .values(cat)
            .get_result(conn)
            .expect("Error saving new cat")
    }

    pub fn read(conn: &mut PgConnection) -> Vec<Self> {
        cats::table
            .order(cats::id.desc())
            .load::<Self>(conn)
            .expect("Error reading cats")
    }

    pub fn read_one(conn: &mut PgConnection, id: i32) -> Option<Self> {
        cats::table.find(id).first(conn).ok()
    }

    pub fn update(&self, conn: &mut PgConnection, new_cat: &Self) -> Self {
        diesel::update(cats::table.find(self.id))
            .set(new_cat)
            .get_result(conn)
            .expect("Error updating cat")
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> usize {
        diesel::delete(cats::table.find(id))
            .execute(conn)
            .expect("Error deleting cat")
    }
}
