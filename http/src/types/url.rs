///////////////////////////////////////////////////////////////
/// Http Url
/// RFC-2616 3.2.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-3.2.2
///////////////////////////////////////////////////////////////
/// URL = "http:" "//" host [: port] [abs_path ["?" query]] ["#" hash]
///////////////////////////////////////////////////////////////
use std::collections::HashMap;
use urlencoding::{encode, decode};

#[derive(Debug)]
pub struct Search(HashMap<String, String>);

#[allow(dead_code)]
impl Search {
    pub fn new() -> Self {
        Self {
            0: HashMap::new()
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.0.insert(
            key.to_string(), 
            value.to_string()
        );
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn remove(&mut self, key:&str) -> Option<String> {
        self.0.remove(key)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Debug)]
pub struct Url {
    pub hash_value: String,
    pub hostname: String,
    pub password: Option<String>,
    pub pathname: String,
    pub port: u16,
    pub protocol: String,
    pub search: Search,
    pub username: Option<String>
}

