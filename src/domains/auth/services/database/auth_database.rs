use std::env;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;

use crate::domains::auth::dto::auth_dto::NewAuth;
use crate::domains::auth::services::jwt_service::create_tokens;
use crate::schema::auths;

pub fn create(conn: &mut PgConnection, user_id: &i32) -> NewAuth {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the expiration time from the environment variable

    let secret = env::var("ACCESS_TOKEN_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("ACCESS_TOKEN_EXPIRES_IN")
        .expect("EXPIRATION_TIME must be set")
        .parse::<i64>()
        .expect("EXPIRATION_TIME must be a valid integer");

    let tokens = create_tokens(*user_id, secret);

    let new_auth = NewAuth {
        user_id: user_id.to_owned(),
        access_token: tokens.0,
        refresh_token: tokens.1,
        expiration: Some(chrono::Utc::now().naive_utc() + chrono::Duration::seconds(expiration)),
        is_valid: true,
        auth_provider: "EMAIL".to_string(),
    };

    diesel::insert_into(auths::table)
        .values(&new_auth)
        .execute(conn)
        .expect("Error saving new auth");

    // find the existing access Token if found invalidate the access Token

    new_auth
}

pub fn make_token_invalid_by_user_id(conn: &mut PgConnection, user_id: &i32) -> usize {
    diesel::update(auths::table.filter(auths::user_id.eq(user_id)))
        .set(auths::is_valid.eq(false))
        .execute(conn)
        .expect("Error updating auth")
}
