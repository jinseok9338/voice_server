use crate::{
    database::postgres_pool::Db,
    domains::{
        auth::{
            dto::auth_dto::{AuthRequest, AuthResponse},
            services::{
                auth_service::AuthService, database::auth_database::make_token_invalid_by_user_id,
                jwt_service::decode_access_token,
            },
        },
        user::{dto::new_user_dto::NewUser, services::user_service::UserService},
    },
    errors::base_error_messages::BaseError,
};
use actix_web::{post, put, web, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify};
use chrono::Utc;

use serde_json::json;

#[post("/token")]
async fn login(auth: web::Json<AuthRequest>) -> Result<impl Responder, BaseError> {
    let mut conn = Db::connect_to_db();
    let mut user_service = UserService::new(&mut conn);
    let user = user_service.read_one_user_by_user_name(&auth.username);
    match user {
        Some(user) => {
            if verify(
                &auth.password,
                &user.password.expect("No password was found"),
            )
            .expect("Error verifying password")
            {
                let mut auth_service = AuthService::new(&mut conn);
                auth_service.invalidate_token(&user.id);
                let auth_response = auth_service.generate_token(&user.id);

                Ok(HttpResponse::Ok().json(AuthResponse {
                    access_token: auth_response.access_token,
                    refresh_token: auth_response.refresh_token,
                    expiration: auth_response.expiration,
                }))
            } else {
                Err(BaseError::Unauthorized)
            }
        }
        None => Err(BaseError::NotFound("User not found".to_string())),
    }
}

#[put("/token")]
async fn logout(req: HttpRequest) -> Result<impl Responder, BaseError> {
    let auth_header = req.headers().get("Authorization");
    let auth_header = match auth_header {
        Some(auth_header) => auth_header.to_str().unwrap(),
        None => return Err(BaseError::Unauthorized),
    };

    // get the token from the header using actix web HttpRequest
    let token = auth_header.trim_start_matches("Bearer ");
    
    // secret is env.ACCESS_TOKEN_SECRET
    let secret = std::env::var("ACCESS_TOKEN_SECRET");
    let secret = match secret {
        Ok(secret) => secret,
        Err(_) => return Err(BaseError::Unauthorized),
    };
    
    let mut conn = Db::connect_to_db();
    let claim = decode_access_token(token, secret).expect("Error decoding token");
    make_token_invalid_by_user_id(&mut conn, &claim.user_id);
    
    Ok(HttpResponse::Ok().json(json!({"message": "Logout successful"})))
}


#[post("/signup")]
async fn signup(user: web::Json<NewUser>) -> Result<impl Responder, BaseError> {
    let mut user_service_conn = Db::connect_to_db();
    let mut auth_service_conn = Db::connect_to_db();

    let mut user_service = UserService::new(&mut user_service_conn);
    let mut auth_service = AuthService::new(&mut auth_service_conn);
    let user_exists = user_service.read_one_user_by_user_name(&user.username);
    match user_exists {
        Some(_) => Err(BaseError::Conflict("User already exists".to_string())),
        None => {
            let hashed_password = hash(&user.password, 12).expect("Error hashing password");
            let new_user = NewUser {
                username: user.username.clone(),
                password: hashed_password,
                email: user.email.clone(),
                last_login_at: Some(Utc::now().naive_utc()),
                user_image: user.user_image.clone(),
            };
            let created_user = user_service.create_user(&new_user);
            let auth_response = auth_service.generate_token(&created_user.id);
            Ok(HttpResponse::Ok().json(AuthResponse {
                access_token: auth_response.access_token,
                refresh_token: auth_response.refresh_token,
                expiration: auth_response.expiration,
            }))
        }
    }
}
