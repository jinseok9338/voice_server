use std::sync::Arc;

use crate::{
    domains::{
        auth::{
            dto::auth_dto::{Auth, AuthRequest, AuthResponse, ReissueRequest},
            services::jwt_service::Claims,
        },
        services::AppStateServices,
        user::dto::user_dto::{NewUser, User},
    },
    errors::base_error_messages::{BaseError, BaseErrorMessages},
};
use actix_web::{post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify};

use serde_json::json;

#[post("/token")]
async fn login(
    auth: web::Json<AuthRequest>,
    data: web::Data<Arc<AppStateServices>>,
) -> Result<impl Responder, BaseError> {
    let data = data.into_inner();
    let mut user_service = data.user_service();

    let user = user_service.read_one_user_by_user_name(&auth.username);

    match user {
        Some(user) => {
            if verify(
                &auth.password,
                &user.password.expect("No password was found"),
            )
            .expect("Error verifying password")
            {
                match user_service.update_last_login_at(&user.id) {
                    Ok(_) => {}
                    Err(_) => return Err(BaseError::InternalServerError),
                }
                let mut auth_service = data.auth_service();

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
        None => Err(BaseError::NotFound(BaseErrorMessages::new(
            "User not found".to_string(),
            1,
        ))),
    }
}

#[put("/token")]
async fn logout(
    req: HttpRequest,
    data: web::Data<AppStateServices>,
) -> Result<impl Responder, BaseError> {
    // Access the claims from the request extensions
    let claims = req.extensions();
    let claims = claims.get::<Claims>();
    let claims = match claims {
        Some(claims) => claims,
        None => {
            return Err(BaseError::NotFound(BaseErrorMessages::new(
                "User not found".to_string(),
                1,
            )))
        }
    };
    let mut auth_service = data.auth_service();
    let user_id = claims.user_id;

    auth_service.invalidate_token(&user_id);

    Ok(HttpResponse::Ok().json(json!({"message": "Logout successful"})))
}

#[put("/token/reissue")]
async fn reissue_token(
    _req: HttpRequest,
    reissue_request: web::Json<ReissueRequest>,
    data: web::Data<AppStateServices>,
) -> Result<impl Responder, BaseError> {
    let mut auth_service = data.auth_service();
    let mut user_service = data.user_service();

    let user_auth: Option<Auth> =
        auth_service.get_auth_by_refresh_token(&reissue_request.refresh_token);
    match user_auth {
        // if user generate new Token associated with user_id
        Some(user_auth) => {
            let user = user_service.read_one_user(user_auth.user_id.unwrap());
            match user {
                Some(user) => {
                    auth_service.invalidate_token(&user.id);
                    let auth_response = auth_service.generate_token(&user.id);
                    Ok(HttpResponse::Ok().json(AuthResponse {
                        access_token: auth_response.access_token,
                        refresh_token: auth_response.refresh_token,
                        expiration: auth_response.expiration,
                    }))
                }
                None => Err(BaseError::NotFound(BaseErrorMessages::new(
                    "User not found".to_string(),
                    1,
                ))),
            }
        }
        None => Err(BaseError::NotFound(BaseErrorMessages::new(
            "User not found".to_string(),
            1,
        ))),
    }
}

#[utoipa::path(
    post,
    responses(
        (status = 200, response = inline(AuthResponse)),
    ),

    request_body(
        content = inline(NewUser),
        content_type = "application/json",
    ),
    context_path = "/auth",
   
)]
#[post("/signup")]
async fn signup(
    user: web::Json<NewUser>,
    data: web::Data<Arc<AppStateServices>>,
) -> Result<impl Responder, BaseError> {
    let data = data.into_inner();
    let mut user_service = data.user_service();
    let mut auth_service = data.auth_service();
    let user_exists = user_service.read_one_user_by_user_name(&user.username);
    match user_exists {
        Some(_) => Err(BaseError::Conflict(BaseErrorMessages::new(
            "User already exists".to_string(),
            2,
        ))),
        None => {
            let password = user.password.clone();
            let hashed_password = hash(password, 12).expect("Error hashing password");

            let new_user: User = User::new(
                user.username.clone(),
                hashed_password,
                user.email.clone(),
                user.user_image.clone(),
            );

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
