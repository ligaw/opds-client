use reqwest::{header, Client, StatusCode};
use thiserror::Error;

// Defining our own error type for the OPDS client
#[derive(Debug, Error)]
pub enum OpdsClientError {
    #[error("HTTP request failed")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("Unexpected HTTP status code: {0}")]
    UnexpectedHttpStatus(StatusCode),
}

pub enum Auth {
    Basic(String, String),
    Bearer(String),
}

pub struct OpdsClient {
    client: Client,
    base_url: String,
    auth_type: Option<Auth>,
}

impl OpdsClient {
    pub fn new(base_url: String, auth_type: Option<Auth>) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url,
            auth_type,
        }
    }

    fn request(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let request_builder = self.client.get(url);
        match &self.auth_type {
            Some(Auth::Basic(username, password)) => {
                request_builder.basic_auth(username, Some(password))
            }
            Some(Auth::Bearer(token)) => {
                request_builder.header(header::AUTHORIZATION, format!("Bearer {}", token))
            }
            None => request_builder,
        }
    }

    pub async fn fetch_feed(&self, path: &str) -> Result<String, OpdsClientError> {
        let response = self.request(&path).send().await?;

        if !response.status().is_success() {
            return Err(OpdsClientError::UnexpectedHttpStatus(response.status()));
        }

        Ok(response.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_fetch_feed() {
        let server = MockServer::start();

        let opds_mock = server.mock(|when, then| {
            when.method(GET).path("/catalog");
            then.status(200).body("<feed></feed>");
        });
        let client = OpdsClient::new(server.base_url(), None);

        let response = client.fetch_feed("/catalog").await;
        opds_mock.assert();
        assert!(response.is_ok());
    }
}
