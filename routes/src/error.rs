use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use thiserror::Error;
use paperclip::actix::api_v2_errors;

pub type WebResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
#[api_v2_errors]
pub enum Error {
    #[error("{0}")]
    Dal(#[from] dal::Error),
    #[error("{0}")]
    Api(#[from] api::Error),
    #[error("Not found: {0}")]
    NotFound(&'static str),
    #[error("{0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("{0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Dal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Api(x) => match x {
                api::Error::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
                api::Error::Reqwest(_) => StatusCode::FAILED_DEPENDENCY,
                api::Error::Upstream(_, _) => StatusCode::FAILED_DEPENDENCY,
            },
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Base64Decode(_) => StatusCode::BAD_REQUEST,
            Self::FromUtf8(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::Dal(_) => HttpResponse::build(self.status_code()).body("Internal server error"),
            _ => HttpResponse::build(self.status_code())
                .insert_header(("Content-Type", "text/plain"))
                .body(format!("{self}"))
        }
    }
}