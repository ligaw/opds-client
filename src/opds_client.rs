use crate::auth::Auth;
use crate::error::OpdsClientError;
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header;
use xml::reader::{EventReader, XmlEvent};

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

    fn parse_events(events: EventReader<&[u8]>) {
        let mut depth = 0;
        for e in events {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{:spaces$}+{name}", "", spaces = depth * 2);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = depth * 2);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                _ => {}
            }
        }
    }

    fn parse_response(response: Response) {
        let body = response.bytes().unwrap();
        let events = EventReader::new(&body[..]);
        OpdsClient::parse_events(events);
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

    pub fn fetch_feed(&self, path: &str) -> Result<String, OpdsClientError> {
        let request_builder = self.request(path);
        let response = request_builder.send()?;

        if !response.status().is_success() {
            return Err(OpdsClientError::UnexpectedHttpStatus(response.status()));
        }

        OpdsClient::parse_response(response);
        Ok(String::from("Hello"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    use std::{fs::File, io::Read};

    #[test]
    fn test_fetch_feed() {
        let server = MockServer::start();

        let opds_mock = server.mock(|when, then| {
            when.method(GET).path("/catalog");
            then.status(200).body("<feed></feed>");
        });
        let client = OpdsClient::new(server.base_url(), None);

        let response = client.fetch_feed("/catalog");
        opds_mock.assert();
        assert!(response.is_ok());
    }

    #[test]
    fn test_parse() {
        let mut file = File::open("tests/resources/catalog.xml").expect("Failed to open XML file");
        let mut content = Vec::new();
        file.read_to_end(&mut content).expect("Failed to read file");

        OpdsClient::parse_events(EventReader::new(&content[..]));
    }
}
