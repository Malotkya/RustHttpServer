///////////////////////////////////////////////////////////////
/// Http Url
/// RFC-2616 3.2.2
/// https://datatracker.ietf.org/doc/html/rfc2616#section-3.2.2
///////////////////////////////////////////////////////////////
/// URL = "http:" "//" host [: port] [abs_path ["?" query]] ["#" hash]
///////////////////////////////////////////////////////////////
use std::collections::HashMap;

const HTTPS_DEFAULT:u16 = 443;

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
pub enum Protocol {
    Http,
    Https
}

impl TryFrom<String> for Protocol {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "HTTPS" => Ok(Self::Http),
            "HTTP" => Ok(Self::Http),
            _ => Err(format!("{} is not an accepted protocol!", value))
        }
    }
}

#[derive(Debug)]
pub enum Hostname {
    Ipv4(u8, u8, u8, u8),
    Text(String),
    None
}

fn get_ipv4(value:&str) -> Option<Hostname> {
    let list: Vec<_> = value.split(".").collect();

    if list.len() != 4 {
        return None
    }

    let first:u8 = match list.get(0) {
        Some(text) =>{
            match text.parse() {
                Ok(number) => number,
                Err(_) => return None
            }
        },
        None => return None
    };

    let second:u8 = match list.get(0) {
        Some(text) =>{
            match text.parse() {
                Ok(number) => number,
                Err(_) => return None
            }
        },
        None => return None
    };

    let third:u8 = match list.get(0) {
        Some(text) =>{
            match text.parse() {
                Ok(number) => number,
                Err(_) => return None
            }
        },
        None => return None
    };

    let fourth:u8 = match list.get(0) {
        Some(text) =>{
            match text.parse() {
                Ok(number) => number,
                Err(_) => return None
            }
        },
        None => return None
    };

    Some(Hostname::Ipv4(first, second, third, fourth))
}

impl From<String> for Hostname {
    fn from(value: String) -> Hostname {
        get_ipv4(&value).unwrap_or(
            Hostname::Text(value)
        )
    }
}



#[derive(Debug)]
pub struct Url {
    pub hashvalue: String,
    pub hostname: Hostname,
    password: Option<String>,
    path: Vec<String>,
    pub port: u16,
    pub protocol: Protocol,
    pub search: Search,
    username: Option<String>
}

impl Url {
    pub fn new(hostname:Hostname, port:u16, path:Vec<String>) -> Self {
        Self {
            hostname, port, path,
            protocol: if port == HTTPS_DEFAULT {
                Protocol::Https
            } else {
                Protocol::Http
            },
            username: None,
            password: None,
            search: Search::new(),
            hashvalue: String::new()
        }
    }

    pub fn empty(port:u16, path:&str) -> Self {
        Self::new(
            Hostname::Ipv4(127, 0, 0, 1),
            port,
            path.split("/").map(|s|s.to_string()).collect()
        )
    }

    pub fn set_auth(&mut self, username:String, password:String) {
        self.username = Some(username);
        self.password = Some(password);
    }

    pub fn pathname(&self) -> String {
        String::from("/") + 
            &self.path.join("/")
    }
}

pub trait ToUrl {
    type Error;
    fn to_url(&self, default_hostname:Hostname, default_port:u16) -> Result<Url, Self::Error>;
}
