use std::io::BufReader;
use std::net::TcpStream;
use std::collections::HashMap;
use crate::Url;

pub mod uri;

pub struct Headers(HashMap<String, String>);

#[allow(dead_code)]
impl Headers {
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

pub struct Request<PARAM, STREAM = TcpStream> {
    pub url: Url,
    pub method: Method,
    pub headers: HashMap<String, String>,
    body: BufReader<STREAM>,
    pub param: PARAM
}

#[allow(dead_code)]
impl<PARAM, STREAM> Request<PARAM, STREAM> {
    pub fn new(url:Url, method:Method, headers:HashMap<String, String>, body: BufReader<STREAM>, param:PARAM)->Self{
        Self {
            url, method, headers, body, param
        }
    }

    pub fn body(&self)->&[u8] {
        self.body.buffer()
    }
}