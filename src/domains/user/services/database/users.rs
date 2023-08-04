use crate::{
    domains::user::dto::user_dto::{User, UserWithOutPassword},
    schema::users,
};

use chrono::Utc;
use diesel::{
    query_dsl::methods::OrderDsl, ExpressionMethods, PgConnection, PgTextExpressionMethods,
    QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

pub fn create(conn: &mut PgConnection, user: &User) -> User {
    diesel::insert_into(users::table)
        .values(user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn _read(conn: &mut PgConnection) -> Vec<User> {
    OrderDsl::order(users::table, users::id.desc())
        .load::<User>(conn)
        .expect("Error reading users")
}

pub fn search_users_in_db(conn: &mut PgConnection, search_term: &str) -> Vec<UserWithOutPassword> {
    let search_term = format!("%{}%", search_term);
    let users = users::table
        .filter(users::username.ilike(&search_term))
        .or_filter(users::email.ilike(&search_term))
        .load::<User>(conn)
        .expect("Error reading users");

    // convert to UserWithOutPassword
    let users = users
        .into_iter()
        .map(|user| User::user_without_password(&user))
        .collect::<Vec<UserWithOutPassword>>();
    users
}

pub fn read_one(conn: &mut PgConnection, id: Uuid) -> Option<UserWithOutPassword> {
    let user: Option<User> = users::table.find(id).first(conn).ok();
    match user {
        Some(user) => Some(User::user_without_password(&user)),
        None => None,
    }
}

pub fn read_one_user_by_user_id_with_password(conn: &mut PgConnection, id: Uuid) -> Option<User> {
    users::table.find(id).first(conn).ok()
}

pub fn read_one_user_by_name(conn: &mut PgConnection, user_name: &str) -> Option<User> {
    users::table
        .filter(users::username.eq(user_name))
        .first(conn)
        .ok()
}

pub fn update_last_login_at_to_database(conn: &mut PgConnection, user_id: &Uuid) {
    diesel::update(users::table.find(user_id))
        .set(users::last_login_at.eq(Utc::now().naive_utc()))
        .execute(conn)
        .expect("Error updating user");
}

pub fn update_one(conn: &mut PgConnection, id: Uuid, user: &User) -> UserWithOutPassword {
    let user = User::updated_user(
        user,
        Some(user.username.clone()),
        Some(user.email.clone()),
        user.last_login_at,
        user.user_image.clone(),
    );

    let updated_user: User = diesel::update(users::table.find(id))
        .set(user)
        .get_result(conn)
        .expect("Error updating user");

    User::user_without_password(&updated_user)
}

pub fn delete_one(conn: &mut PgConnection, id: Uuid) -> usize {
    diesel::delete(users::table.find(id))
        .execute(conn)
        .expect("Error deleting user")
}
