use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use thiserror::Error;

pub type WebResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Dal(#[from] dal::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Dal(_) => StatusCode::INTERNAL_SERVER_ERROR
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