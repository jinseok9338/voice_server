// this is where the basic CRUD operations are defineduse diesel::prelude::*;
use crate::{
    domains::user::dto::{new_user_dto::NewUser, user_dto::User},
    schema::users,
};
use diesel::{
    query_dsl::methods::OrderDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

pub fn create(conn: &mut PgConnection, user: &NewUser) -> User {
    diesel::insert_into(users::table)
        .values(user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn read(conn: &mut PgConnection) -> Vec<User> {
    OrderDsl::order(users::table, users::id.desc())
        .load::<User>(conn)
        .expect("Error reading users")
}

pub fn read_one(conn: &mut PgConnection, id: i32) -> Option<User> {
    users::table.find(id).first(conn).ok()
}

pub fn update_one(conn: &mut PgConnection, id: i32, new_cat: &User) -> User {
    diesel::update(users::table.find(id))
        .set(new_cat)
        .get_result(conn)
        .expect("Error updating user")
}

pub fn delete_one(conn: &mut PgConnection, id: i32) -> usize {
    diesel::delete(users::table.find(id))
        .execute(conn)
        .expect("Error deleting user")
}
