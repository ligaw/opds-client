use crate::auth::Auth;
use crate::connection::Connection;
use crate::error::Error;
use serde::Deserialize;

pub struct Server {
    client: Connection,
}

impl Server {
    pub fn new(base_url: String, auth_type: Option<Auth>) -> Self {
        let client = Connection::new(base_url, auth_type);
        Self { client }
    }

    pub fn catalog(&self) -> Result<OpdsEntry, Error> {
        let catalog = self.client.get_xml("/catalog")?;
        Server::parse(&catalog[..])
    }

    pub fn get_xml(&self, path: String) -> Result<OpdsEntry, Error> {
        let entries = self.client.get_xml(&path[..])?;
        Server::parse(&entries[..])
    }

    pub fn parse(xml_string: &str) -> Result<OpdsEntry, Error> {
        let entries = serde_xml_rs::from_str(xml_string);
        match entries {
            Ok(entries) => Ok(entries),
            Err(_) => Err(Error::ParseError()),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        let content =
            fs::read_to_string("tests/resources/root.xml").expect("Failed to open XML file");

        let catalog = Server::parse(&content[..]).expect("Parsing failed");
        assert_eq!(catalog.title, "OPDS Catalog Root Example");
    }
}
