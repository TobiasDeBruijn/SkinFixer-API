use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use paperclip::actix::api_v2_errors;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[api_v2_errors(
    code = 400,
    code = 500,
    code = 429,
)]
pub enum Error {
    #[error("Internal server error")]
    Mysql(#[from] mysql::Error),
    #[error("Envy: {0:?}")]
    Envy(#[from] envy::Error),
    #[error("Refinery: {0:?}")]
    Refinery(#[from] refinery::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServer,
    #[error("Too many requests")]
    TooManyRequests,
    #[error("Internal server error: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Internal server error: {0:?}")]
    Base64(#[from] base64::DecodeError),
    #[error("Internal server error: {0:?}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error")]
    Json(#[from] serde_json::Error),
}

impl Error {
    fn log(&self) {
        if self.status_code().eq(&StatusCode::INTERNAL_SERVER_ERROR) {
            log::warn!("{:?}", self);
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Mysql(_) | Self::Envy(_) | Self::Refinery(_)
            | Self::InternalServer | Self::Reqwest(_) | Self::Json(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) | Self::Base64(_) | Self::FromUtf8(_) => StatusCode::BAD_REQUEST,
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        Self::status_code(self)
    }

    fn error_response(&self) -> HttpResponse {
        self.log();
        HttpResponse::build(Self::status_code(self))
            .body(format!("{}", self))
    }
}