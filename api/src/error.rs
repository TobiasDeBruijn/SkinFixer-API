use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// Reqwest error
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// Too many requests for the provided API name
    #[error("Too many requests: {0}")]
    TooManyRequests(&'static str),
    /// Upstream error
    #[error("Upstream error from {0}: {1}")]
    Upstream(&'static str, String),
}