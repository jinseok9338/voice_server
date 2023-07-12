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
    Unauthorized,
    Conflict(BaseErrorMessages),
    DatabaseError(diesel::result::Error),
    // add other error types here
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {:?}", msg),
            Self::InternalServerError => write!(f, "{:?}", self),
            Self::Unauthorized => write!(f, "{:?}", self),
            Self::Conflict(msg) => write!(f, "Conflict: {:?}", msg),
            Self::DatabaseError(err) => write!(f, "Database error: {:?}", err),
            // add other error types here
        }
    }
}

impl ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound(msg) => HttpResponse::NotFound().json(msg),
            Self::InternalServerError => HttpResponse::InternalServerError().json(
                BaseErrorMessages::new("Internal server error".to_string(), 1),
            ),
            Self::Unauthorized => HttpResponse::Unauthorized()
                .json(BaseErrorMessages::new("Unauthorized".to_string(), 1)),
            Self::Conflict(msg) => HttpResponse::Conflict().json(msg),
            Self::DatabaseError(err) => {
                let error_message = format!("{:?}", err);
                let wrapper = DatabaseErrorWrapper(error_message);
                HttpResponse::InternalServerError().json(wrapper)
            }
            // add other error types here
        }
    }
}
