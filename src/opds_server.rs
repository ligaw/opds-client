use crate::auth::Auth;
use crate::opds_client::OpdsClient;
use serde::Deserialize;
use std::error::Error;

pub struct OpdsServer {
    client: OpdsClient,
}

impl OpdsServer {
    pub fn new(base_url: String, auth_type: Option<Auth>) -> Self {
        let client = OpdsClient::new(base_url, auth_type);
        Self { client }
    }

    pub fn catalog(&self) -> OpdsEntry {
        let opds_entry = self
            .client
            .get_xml("/catalog")
            .expect("Failed to get catalog");
        Self::parse(&opds_entry)
    }

    pub fn parse(xml_string: &str) -> OpdsEntry {
        let opds_entry: OpdsEntry =
            serde_xml_rs::from_str(xml_string).expect("Failed to parse XML");
        opds_entry
    }
}

#[derive(Deserialize, Debug)]
pub struct OpdsEntry {
    pub title: String,
    pub id: String,
    pub author: Option<Author>,
    #[serde(rename = "link")]
    pub links: Vec<Link>,
    #[serde(rename = "entry")]
    pub entries: Option<Vec<OpdsEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    pub rel: String,
    pub href: String,
}

#[derive(Debug)]
struct OpdsEntryParsingError {
    source_error: Box<dyn Error>,
    xml: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        let content =
            fs::read_to_string("tests/resources/catalog.xml").expect("Failed to open XML file");

        match OpdsServer::parse(&content[..]) {
            Ok(opds_entry) => {
                println!("{:#?}", opds_entry);
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}
