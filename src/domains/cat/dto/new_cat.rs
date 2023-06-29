use diesel::Insertable;
use serde_derive::Deserialize;

use crate::schema::cats;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = cats)]
pub struct NewCat {
    pub name: String,
    pub age: i32,
    pub breed: String,
    pub color: String,
    pub weight: f64,
    pub image: String,
}
