use actix_http::header::{HeaderName, HeaderValue};
use actix_web::{error::ResponseError, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseErrorMessages {
    pub message: String,
    pub code: u16,
}

impl BaseErrorMessages {
    pub const fn new(message: String, code: u16) -> Self {
        Self { message, code }
    }
}

#[derive(Debug, Serialize)]
struct DatabaseErrorWrapper(String);

#[derive(Debug)]
pub enum BaseError {
    NotFound(BaseErrorMessages),
    InternalServerError,
    BadRequest(BaseErrorMessages),
    Unauthorized,
    Conflict(BaseErrorMessages),
    DatabaseError(diesel::result::Error),
    //web socket Connection Error
    ConnectionError(BaseErrorMessages), // add other error types here
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {:?}", msg),
            Self::InternalServerError => write!(f, "{:?}", self),
            Self::Unauthorized => write!(f, "{:?}", self),
            Self::Conflict(msg) => write!(f, "Conflict: {:?}", msg),
            Self::DatabaseError(err) => write!(f, "Database error: {:?}", err),
            Self::BadRequest(msg) => write!(f, "Bad request: {:?}", msg),
            Self::ConnectionError(msg) => write!(f, "Connection error: {:?}", msg),
            // add other error types here
        }
    }
}

impl From<actix_web::Error> for BaseError {
    fn from(_: actix_web::Error) -> Self {
        BaseError::InternalServerError
    }
}

impl ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse {
        let mut response = match self {
            Self::NotFound(msg) => HttpResponse::NotFound().json(msg),
            Self::InternalServerError => HttpResponse::InternalServerError().json(
                BaseErrorMessages::new("Internal server error".to_string(), 1),
            ),
            Self::Unauthorized => {
                let error_message = BaseErrorMessages::new("Unauthorized".to_string(), 401);
                HttpResponse::Unauthorized().json(error_message)
            }
            Self::Conflict(msg) => HttpResponse::Conflict().json(msg),
            Self::DatabaseError(err) => {
                let error_message = format!("{:?}", err);
                let wrapper = DatabaseErrorWrapper(error_message);
                HttpResponse::InternalServerError().json(wrapper)
            } // add other error types here
            Self::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            Self::ConnectionError(msg) => HttpResponse::InternalServerError().json(msg),
        };

        // Add headers
        let headers = response.headers_mut();
        headers.insert(
            HeaderName::from_static("access-control-allow-origin"),
            HeaderValue::from_static("*"),
        );
        headers.insert(
            HeaderName::from_static("access-control-allow-methods"),
            HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
        );
        headers.insert(
            HeaderName::from_static("access-control-allow-headers"),
            HeaderValue::from_static("Authorization, Accept, Content-Type"),
        );

        response
    }
}
