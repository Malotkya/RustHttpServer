/// Http Url Format
/// 
/// ["http:"|"https:" "//" host [ ":" port ]] [ abs_path [ "?" query ]? ["#" hash]?]
/// 
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
pub enum UrlParseError {
    NoHostName,
    InvalidPortNumber(String),
    IssueDecoding(String),
    LeftOverChars(String)
}

impl std::fmt::Display for UrlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoHostName => write!(f, "Unable to find host name in string!"),
            Self::InvalidPortNumber(value) => write!(f, "{} is not a valid port number!", value),
            Self::IssueDecoding(value) => write!(f, "Unable to decode '{}'!", value),
            Self::LeftOverChars(value) => write!(f, "'{}' found at end of url string!", value)
        }
    }
}

#[derive(Debug)]
pub struct Url {
    hash_value: String,
    hostname: String,
    password: String,
    pathname: String,
    port: u16,
    protocol: Vec<String>,
    pub search: Search,
    username: String
}

impl Url {
    pub fn parse(mut string: &str) -> Result<Url, UrlParseError> {
        let protocol:Vec<String>;
        match string.find("//") {
            Some(index) => {
                protocol = string[..index].split(":")
                .map(|str|{
                    String::from(str).to_lowercase()
                }).collect();
                string = &string[index+2..];
            }
            None => {
                protocol = Vec::new()
            }
        };

        let username: String;
        let password: String;
        match string.find("@") {
            Some(index) => {
                let list: Vec<&str> = string[..index].split(":").collect();
                username = String::from( *list.get(0).unwrap_or(&"") );
                password = String::from( *list.get(1).unwrap_or(&"") );
            },
            None => {
                username = String::new();
                password = String::new();
            }
        }

        let mut find_port:  bool = false;
        let mut find_path:  bool = false;
        let mut find_hash:  bool = false;
        let mut find_search:bool = false;
        let hostname:String;
        match string.find(":") {
            Some(index) => {
                find_port = true;
                hostname = String::from(&string[..index]);
                string = &string[index+1..];
            },
            None => {

                match string.find("/") {
                    Some(index) => {
                        find_path = true;
                        hostname = String::from(&string[..index]);
                        string = &string[index+1..];
                    }
                    None => {

                        match string.find("#") {
                            Some(index) => {
                                find_hash = true;
                                hostname = String::from(&string[..index]);
                                string = &string[index+1..];
                            },
                            None => {

                                match string.find("?") {
                                    Some(index) => {
                                        find_search = true;
                                        hostname = String::from(&string[..index]);
                                        string = &string[index+1..];
                                    },
                                    None => {

                                        if string.len() > 0 {
                                            hostname = String::from(string);
                                            string = "";
                                        } else {

                                            return Err(UrlParseError::NoHostName)
                                        }
                                    }
                                } //Find start of search
                            }
                        } // Find start of hash
                    }
                } //Find start of pathname
            }
        } //Find start of port

        let port:u16;
        if find_port {
            let port_str: String;
            match string.find("/") {
                Some(index) => {
                    find_path = true;
                    port_str = String::from(&string[..index]);
                    string = &string[index+1..]
                },
                None => {

                    match string.find("#") {
                        Some(index) => {
                            find_hash = true;
                            port_str = String::from(&string[..index]);
                            string = &string[index+1..]
                        },
                        None => {
                            match string.find("?") {
                                Some(index) => {
                                    find_search = true;
                                    port_str = String::from(&string[..index]);
                                    string = &string[index+1..]
                                },
                                None => {
                                    port_str = String::from(string);
                                    string = "";
                                }
                            } //End find Search
                        }
                    } // End find Hash
                }
            } //End find Pathanme

            match port_str.parse::<u16>() {
                Ok(value) => {
                    port = value;
                },
                Err(e) => {
                    return Err(UrlParseError::InvalidPortNumber(port_str))
                }
            }
        } else {
            match protocol.last().unwrap_or(&String::new()).as_str() {
                "http" => {
                    port = 80;
                },
                "https" => {
                    port = 443;
                },
                _ => {
                    port = 0;
                }
            }
        }

        let pathname:String;
        if find_path {
            match string.find("#") {
                Some(index) => {
                    find_hash = true;
                    pathname = String::from(&string[..index]);
                    string = &string[index+1..]
                },
                None => {

                    match string.find("?") {
                        Some(index) => {
                            find_search = true;
                            pathname = String::from(&string[..index]);
                            string = &string[index+1..]
                        },
                        None => {
                            if string.len() > 0 {
                                pathname = String::from(string);
                                string = "";
                            } else {
                                pathname = String::from("/");
                            }
                        }
                    }
                }
            } // End find Hash
        } else {
            pathname = String::from("/");
        }

        let hash_value:String;
        if find_hash  {
            match string.find("?") {
                Some(index) => {
                    find_search = true;
                    hash_value = String::from(&string[1..index]);
                    string = &string[index+1..]
                },
                None => {
                    hash_value = String::from(string);
                    string = "";
                }
            }
        } else {
            hash_value = String::new();
        }

        let mut search = Search::new();
        if find_search  {
            string = &string[1..];

            for lines in string.split("&") {
                let list: Vec<&str> = lines.split("=").collect();

                let key_str = list.get(0).unwrap_or(&"").trim();
                let key = decode(key_str);

                if key.is_err() {
                    return Err(UrlParseError::IssueDecoding(String::from(key_str)));
                }

                let value_str = list.get(1).unwrap_or(&"").trim();
                let value = decode(value_str);
                
                if value.is_err() {
                    return Err(UrlParseError::IssueDecoding(String::from(value_str)));
                }

                search.set(&key.unwrap().to_string(), &value.unwrap().to_string());
            }

            string = "";
        }

        if string.len() > 0 {
            Err(UrlParseError::LeftOverChars(String::from(string)))
        } else {
            Ok (
                Url {
                    protocol, hash_value, hostname, pathname, port, search,
                    username, password
                }
            )
        }
    }

    pub fn get_hash_value(&self) -> String {
        self.hash_value.clone()
    }
    
    pub fn get_hash(&self) -> String {
        format!("#{}", self.hash_value)
    }

    pub fn set_hash<'a>(&mut self, string:&'a str) {
        let mut list = string.chars();
        
        let first = list.nth(0);
        if first.is_some() && first.unwrap() == '#' {
            list.next();
        }

        self.hash_value = encode(list.as_str()).to_string();
    }

    pub fn get_host(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn get_host_name(&self) -> String {
        self.hostname.clone()
    }

    pub fn set_host(&mut self, value: &str) {
        self.hostname = encode(value).to_string()
    }

    pub fn get_href(&self) -> String {
        self.to_string()
    }

    pub fn get_origin(&self) -> String {
        let start = self.protocol.last();
        let mut origin:String = match start {
            Some(protocol) => {
                format!("{}:", protocol)
            },
            None => String::new()
        };

        origin += &self.hostname;
        origin
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn set_password(&mut self, string: &str) {
        self.password = String::from(string)
    }

    pub fn get_pathname(&self) -> String {
        self.pathname.clone()
    }

    pub fn set_pathname(&mut self, string:&str) {
        if string.is_empty() {
            self.pathname = String::from("/");
        } else {
            self.pathname = format!("/{}", string
                .split("/")
                .filter_map(|str|{
                    if str.is_empty() {
                        None
                    } else {
                        Some(encode(str).to_string())
                    }
                }).collect::<Vec<String>>()
                .join("/"));
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn set_port(&mut self, value: u16) {
        self.port = value;
    }

    pub fn get_protocol(&self) -> String {
        format!("{}:", self.protocol.join(":"))
    }

    pub fn set_protocol(&mut self, string:&str) {
        self.protocol = string.split(":")
            .filter_map(|str|{
                if str.is_empty() {
                    None
                } else {
                    Some(str.to_string())
                }
            }).collect::<Vec<String>>();
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn set_username(&mut self, string: &str) {
        self.username = String::from(string);
    }

    pub fn auth(&self) -> bool {
        !self.username.is_empty()
    }

}

impl ToString for Url {
    fn to_string(&self) -> String {
        todo!()
    }
}