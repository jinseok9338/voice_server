use actix_http::HttpMessage;
use actix_web::{delete, get, put, web, HttpRequest, HttpResponse, Responder};

use crate::{
    database::postgres_pool::Db,
    domains::{
        auth::services::{
            auth_service::AuthService,
            jwt_service::{decode_access_token, Claims},
        },
        user::{
            dto::user_dto::{User, UserRequest},
            services::user_service::UserService,
        },
    },
    errors::base_error_messages::{BaseError, BaseErrorMessages},
};

#[get("/me")]
async fn get_me(req: HttpRequest) -> Result<impl Responder, BaseError> {
    let claims = req.extensions();
    let claims = claims.get::<Claims>();

    // Check if claims exist and get the user_id

    let claims = match claims {
        Some(claims) => claims,
        None => {
            return Err(BaseError::NotFound(BaseErrorMessages::new(
                "User not found".to_string(),
                1,
            )))
        }
    };

    let user_id = claims.user_id;

    let mut conn = Db::connect_to_db();

    let mut service = UserService::new(&mut conn);
    let user = service.read_one_user(user_id);
    user.map_or_else(
        || {
            Err(BaseError::NotFound(BaseErrorMessages::new(
                "User not found".to_string(),
                1,
            )))
        },
        |user| Ok(HttpResponse::Ok().json(user)),
    )
}

#[put("/me")]
async fn update_me(
    req: HttpRequest,
    update_user: web::Json<UserRequest>,
) -> Result<impl Responder, BaseError> {
    let claims = req.extensions();
    let claims = claims.get::<Claims>();

    // Check if claims exist and get the user_id

    let claims = match claims {
        Some(claims) => claims,
        None => {
            return Err(BaseError::NotFound(BaseErrorMessages::new(
                "User not found".to_string(),
                1,
            )))
        }
    };

    let user_id = claims.user_id;
    let mut conn = Db::connect_to_db();
    let mut service = UserService::new(&mut conn);
    let existing_user = service.reat_one_user_by_id(user_id);
    let existing_user = match existing_user {
        Some(existing_user) => existing_user,
        None => {
            return Err(BaseError::NotFound(BaseErrorMessages::new(
                "User not found".to_string(),
                1,
            )))
        }
    };
    let updated_user = User::updated_user(
        &existing_user,
        update_user.username.clone(),
        update_user.email.clone(),
        update_user.last_login_at,
        update_user.user_image.clone(),
    );
    let user = service.update_user(user_id, &updated_user);
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/me")]
async fn delete_me(req: HttpRequest) -> Result<impl Responder, BaseError> {
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

    let claim = decode_access_token(token, secret);
    let claim = match claim {
        Ok(claim) => claim,
        Err(_) => return Err(BaseError::Unauthorized),
    };
    let mut user_service_conn = Db::connect_to_db();
    let mut user_service = UserService::new(&mut user_service_conn);

    let mut auth_service_conn = Db::connect_to_db();
    let mut auth_service = AuthService::new(&mut auth_service_conn);

    // terminate the session with the user_id
    auth_service.invalidate_token(&claim.user_id);
    // delete the user
    user_service.delete_user(claim.user_id);

    Ok(HttpResponse::Ok().json("User deleted"))
}
