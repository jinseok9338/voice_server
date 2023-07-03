use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub last_login_at: Option<NaiveDateTime>,
    pub user_image: Option<String>,
}
