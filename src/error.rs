use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MyError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Rusqlite error: {0}")]
    RusqliteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::ReqwestError(ref e) => {
                HttpResponse::InternalServerError().body(format!("Reqwest error: {}", e))
            }
            MyError::RusqliteError(ref e) => {
                HttpResponse::InternalServerError().body(format!("Database error: {}", e))
            }
        }
    }
}

