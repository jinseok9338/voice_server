use std::env;

use crate::domains::auth::dto::auth_dto::Auth;

use crate::domains::auth::services::jwt_service::create_tokens;
use crate::schema::auths;
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use uuid::Uuid;

pub fn create(conn: &mut PgConnection, user_id: &Uuid) -> Auth {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the expiration time from the environment variable

    let secret = env::var("ACCESS_TOKEN_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("ACCESS_TOKEN_EXPIRES_IN")
        .expect("EXPIRATION_TIME must be set")
        .parse::<i64>()
        .expect("EXPIRATION_TIME must be a valid integer");

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
        .execute(conn)
        .expect("Error saving new auth");

    // find the existing access Token if found invalidate the access Token

    new_auth
}

pub fn make_token_invalid_by_user_id(conn: &mut PgConnection, user_id: &Uuid) -> usize {
    diesel::update(auths::table.filter(auths::user_id.eq(user_id)))
        .set(auths::is_valid.eq(false))
        .execute(conn)
        .expect("Error updating auth")
}

pub fn get_auth_by_token(conn: &mut PgConnection, token: &str) -> Option<Auth> {
    let auth: Option<Auth> = auths::table
        .filter(auths::access_token.eq(token))
        .filter(auths::is_valid.eq(true))
        .filter(auths::expiration.gt(chrono::Utc::now().naive_utc()))
        .first::<Auth>(conn)
        .optional()
        .expect("Error loading auth");

    auth
}
