use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable,AsChangeset};
use serde::{Deserialize, Serialize};

use crate::schema::users;



#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub last_login_at: Option<NaiveDateTime>,
    pub user_image: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
