

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable,AsChangeset};
use serde::{Deserialize, Serialize};

use crate::schema::cats;

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
