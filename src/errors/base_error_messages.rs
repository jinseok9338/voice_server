use actix_web::{error::ResponseError, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum BaseError {
    NotFound(String),
    InternalServerError,
    Unauthorized,
    Conflict(String),
    // add other error types here
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::InternalServerError => write!(f, "Internal server error"),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            // add other error types here
        }
    }
}

impl ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound(msg) => HttpResponse::NotFound().json(msg),
            Self::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal server error")
            } // add other error types here
            Self::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            Self::Conflict(msg) => HttpResponse::Conflict().json(msg),
        }
    }
}
