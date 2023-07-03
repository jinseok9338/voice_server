use actix_web::{delete, get, put, web, HttpRequest, HttpResponse, Responder};

use crate::{
    database::postgres_pool::Db,
    domains::{
        auth::services::{auth_service::AuthService, jwt_service::decode_access_token},
        user::{dto::new_user_dto::NewUser, services::user_service::UserService},
    },
    errors::base_error_messages::BaseError,
};

#[get("/me")]
async fn get_me(req: HttpRequest) -> Result<impl Responder, BaseError> {
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
    let claim = decode_access_token(token, secret);
    let claim = match claim {
        Ok(claim) => claim,
        Err(_) => return Err(BaseError::Unauthorized),
    };
    let mut service = UserService::new(&mut conn);
    let user = service.read_one_user(claim.user_id);
    user.map_or_else(
        || Err(BaseError::NotFound("User not found".to_string())),
        |user| Ok(HttpResponse::Ok().json(user)),
    )
}

#[put("/me")]
async fn update_me(
    req: HttpRequest,
    new_user: web::Json<NewUser>,
) -> Result<impl Responder, BaseError> {
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
    let claim = decode_access_token(token, secret);
    let claim = match claim {
        Ok(claim) => claim,
        Err(_) => return Err(BaseError::Unauthorized),
    };
    let mut service = UserService::new(&mut conn);
    //get new User from request body

    let user = service.update_user(claim.user_id, &new_user);
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
