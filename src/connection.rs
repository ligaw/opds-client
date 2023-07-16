use crate::auth::Auth;
use crate::error::Error;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header;

pub struct Connection {
    client: Client,
    base_url: String,
    auth_type: Option<Auth>,
}

impl Connection {
    pub fn new(base_url: String, auth_type: Option<Auth>) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url,
            auth_type,
        }
    }

    fn request(&self, path: &str) -> RequestBuilder {
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

    pub fn get_xml(&self, path: &str) -> Result<String, Error> {
        let response = self.request(path).send()?;

        Ok(response.text()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_cannot_connect() {}

    #[test]
    fn test_get_xml() {
        let server = MockServer::start();

        let opds_mock = server.mock(|when, then| {
            when.method(GET).path("/catalog");
            then.status(200).body("<feed></feed>");
        });
        let client = Connection::new(server.base_url(), None);

        let response = client.get_xml("/catalog");
        opds_mock.assert();
        assert!(response.is_ok());
    }
}
