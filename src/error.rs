use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("Unexpected HTTP status code: {0}")]
    UnexpectedHttpStatus(StatusCode),
    #[error("Error parsing XML")]
    ParseError(),
}
