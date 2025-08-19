use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[cfg(feature = "actix")]
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};

#[macro_export]
macro_rules! safe_eject {
    ($e:expr, $err_status:expr) => {
        $e.map_err(|x| NanoServiceError::new(x.to_string(), $err_status))
    };
    ($e:expr, $err_status:expr, $message_context:expr) => {
        $e.map_err(|x| {
            NanoServiceError::new(
                format!("{}: {}", $message_context, x.to_string()),
                $err_status,
            )
        })
    };
}

#[derive(Debug, Error, Serialize, Deserialize, PartialEq)]
pub enum NanoServiceErrorStatus {
    #[error("Requested resource was not found.")]
    NotFound,
    #[error("You are forbidden to access requested resource.")]
    Forbidden,
    #[error("Unknown Internal Error.")]
    Unknown,
    #[error("Bad Request.")]
    BadRequest,
    #[error("Conflict.")]
    Conflict,
    #[error("Unauthorized.")]
    Unauthorized,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct NanoServiceError {
    pub message: String,
    pub status: NanoServiceErrorStatus,
}

impl NanoServiceError {
    pub fn new(message: String, status: NanoServiceErrorStatus) -> Self {
        Self { message, status }
    }
}

impl std::fmt::Display for NanoServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "actix")]
impl ResponseError for NanoServiceError {
    fn status_code(&self) -> StatusCode {
        match self.status {
            NanoServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
            NanoServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
            NanoServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            NanoServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
            NanoServiceErrorStatus::Conflict => StatusCode::CONFLICT,
            NanoServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.message.clone())
    }
}
