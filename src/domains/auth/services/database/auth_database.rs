use std::env;

use crate::domains::auth::dto::auth_dto::Auth;

use crate::domains::auth::services::jwt_service::create_tokens;
use crate::schema::auths;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use dotenv::dotenv;
use uuid::Uuid;

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create(conn: &mut Conn, user_id: &Uuid) -> Auth {
    dotenv().ok();

    let secret = env::var("ACCESS_TOKEN_SECRET").expect("JWT_SECRET must be set");

    let tokens = create_tokens(*user_id, secret);
    let new_auth = Auth::new(
        tokens.0,
        tokens.1,
        Some(user_id.to_owned()),
        Some(chrono::Utc::now().naive_utc()),
        true,
        "EMAIL".to_string(),
    );

    diesel::insert_into(auths::table)
        .values(&new_auth)
        .execute(&mut *conn)
        .expect("Error saving new auth");

    new_auth
}

pub fn make_token_invalid_by_user_id(conn: &mut Conn, user_id: &Uuid) -> usize {
    diesel::delete(auths::table.filter(auths::user_id.eq(user_id)))
        .execute(&mut *conn)
        .expect("Error deleting auth")
}

pub fn get_auth_by_token(conn: &mut Conn, token: &str) -> Option<Auth> {
    let auth: Option<Auth> = auths::table
        .filter(auths::access_token.eq(token))
        .filter(auths::is_valid.eq(true))
        .filter(auths::expiration.gt(chrono::Utc::now().naive_utc()))
        .first::<Auth>(&mut *conn)
        .optional()
        .expect("Error loading auth");

    auth
}

pub fn get_auth_by_refresh_token_from_data_base(conn: &mut Conn, token: &str) -> Option<Auth> {
    let auth: Option<Auth> = auths::table
        .filter(auths::refresh_token.eq(token))
        .filter(auths::is_valid.eq(true))
        .first::<Auth>(&mut *conn)
        .optional()
        .expect("Error loading auth");

    auth
}
