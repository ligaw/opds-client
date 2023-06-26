use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpdsClientError {
    #[error("HTTP request failed")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("Unexpected HTTP status code: {0}")]
    UnexpectedHttpStatus(StatusCode),
}
