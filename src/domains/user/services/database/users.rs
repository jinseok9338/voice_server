use crate::{
    domains::user::dto::user_dto::{NewUser, UpdateUser, User, UserWithOutPassword},
    schema::users,
};
use chrono::Utc;
use diesel::{
    query_dsl::methods::OrderDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

pub fn create(conn: &mut PgConnection, user: &NewUser) -> User {
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

pub fn read_one(conn: &mut PgConnection, id: i32) -> Option<UserWithOutPassword> {
    let user: Option<User> = users::table.find(id).first(conn).ok();
    match user {
        Some(user) => Some(UserWithOutPassword {
            id: user.id,
            username: user.username,
            email: user.email,
            last_login_at: user.last_login_at,
            user_image: user.user_image,
            created_at: user.created_at,
            updated_at: user.updated_at,
            tester: user.tester,
        }),
        None => None,
    }
}

pub fn read_one_user_by_name(conn: &mut PgConnection, user_name: &str) -> Option<User> {
    users::table
        .filter(users::username.eq(user_name))
        .first(conn)
        .ok()
}

pub fn update_one(conn: &mut PgConnection, id: i32, user: &UpdateUser) -> UserWithOutPassword {
    let user = UpdateUser {
        username: user.username.to_string(),
        email: user.email.to_string(),
        user_image: user.user_image.to_owned(),
        updated_at: Some(Utc::now().naive_utc()),
        last_login_at: user.last_login_at,
    };

    let updated_user: User = diesel::update(users::table.find(id))
        .set(user)
        .get_result(conn)
        .expect("Error updating user");

    UserWithOutPassword {
        id: updated_user.id,
        username: updated_user.username,
        email: updated_user.email,
        last_login_at: updated_user.last_login_at,
        user_image: updated_user.user_image,
        created_at: updated_user.created_at,
        updated_at: updated_user.updated_at,
        tester: updated_user.tester,
    }
}

pub fn delete_one(conn: &mut PgConnection, id: i32) -> usize {
    diesel::delete(users::table.find(id))
        .execute(conn)
        .expect("Error deleting user")
}
