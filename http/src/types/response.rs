use crate::{Headers, HttpError, HttpStatus, Json};
use std::{collections::LinkedList, fmt};

pub enum Chunk{
    Owned(Vec<u8>),
    String(String)
}

impl Chunk {
    pub fn value<'a>(&'a self) -> &'a [u8] {
        match self {
            Self::Owned(v) => v,
            Self::String(s) => s.as_bytes()
        }
    }
}

impl ToString for Chunk {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Owned(v) => unsafe{ String::from_utf8_unchecked(v.clone()) }
        }
    }
}

impl From<Vec<u8>> for Chunk {
    fn from(value: Vec<u8>) -> Self {
        Self::Owned(value)
    }
}

impl From<String> for Chunk {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Chunk{
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&[u8]> for Chunk {
    fn from(value: &[u8]) -> Self {
        Self::Owned(value.to_vec())
    }
}

impl From<char> for Chunk {
    fn from(value: char) -> Self {
        Self::Owned(vec![value as u8])
    }
}

pub struct Response {
    pub status: HttpStatus,
    pub headers: Headers,
    pub body: LinkedList<Chunk>,
    pub sent: bool
}

#[allow(dead_code)]
impl Response {
    pub fn new(status: HttpStatus, headers: Option<Headers>) -> Self {
        Self {
            status: status,
            headers: headers.unwrap_or(Headers::new()),
            body: LinkedList::new(),
            sent: false
        }
    }

    pub fn write<T>(&mut self, data:T) -> Result<&Self, &'static str> where T: Into<Chunk> {
        if self.sent {
            Err("Response has already been sent!")
        } else {
            self.body.push_back(data.into());
            Ok(self)
        }
        
    }

    pub fn http(&mut self, http:String) ->Result<&Self, &'static str> {
        if self.sent {
            Err("Response has already been sent!")
        } else if let Some(header) = self.headers.get("Content-Type")
            && let Ok(value) = header.ref_str() && value != "application/html" {
            Err("Content-Type header is already set!")
        } else {
            self.headers.set("Content-Type", "application/html");
            self.body.push_back(
                Chunk::String(http)
            );
            Ok(self)
        }
    }

    pub fn json(&mut self, json:&Json) -> Result<&Self, &'static str> {
        if self.sent {
            Err("Response has already been sent!")
        } else if let Some(header) = self.headers.get("Content-Type")
            && let Ok(value) = header.ref_str() && value != "application/json" {
            Err("Content-Type header is already set!")
        } else {
            self.headers.set("Content-Type", "application/json");
            self.body.push_back(
                Chunk::String(json.stringify(None))
            );
            Ok(self)
        }
    }

    pub fn from<T>(body:T) -> Self where T: Into<Chunk>{
        let mut chunks = LinkedList::new();
        chunks.push_back(body.into());
        Self {
            status: HttpStatus::Ok,
            headers: Headers::new(),
            body: chunks,
            sent: false
        }
    }

    pub fn from_http(http:String) -> Self {
        let mut headers = Headers::new();
        headers.set("Content-Type", "application/html");
        let mut body = LinkedList::new();
        body.push_front(Chunk::String(http));

        Self {
            status: HttpStatus::Ok,
            headers, body,
            sent: false
        }
    }

    pub fn from_json(json:&Json) -> Self {
        let mut headers = Headers::new();
        headers.set("Content-Type", "application/json");
        let mut body = LinkedList::new();
        body.push_front(Chunk::String(json.stringify(None)));

        Self {
            status: HttpStatus::Ok,
            headers, body,
            sent: false
        }
    }

    pub fn from_error(error:HttpError) -> Self {
        let HttpError{message, kind } = error;
        let mut body = LinkedList::new();
        body.push_front(
            Chunk::String(message)
        );        

        Self {
            status: kind.into(),
            headers: Headers::new(),
            body,
            sent: false
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        self.body.iter()
            .map(|chunk|chunk.to_string())
            .collect::<Vec<String>>().join("")
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = if self.status.code() >= 400 {
            match self.headers.get("Content-Type") {
                Some(value) => if value.ref_str().unwrap().to_lowercase().find("application").is_some() {
                    self.status.as_str().to_owned()
                } else {
                    self.to_string()
                },
                None => self.to_string()
            }
        } else {
            self.status.as_str().to_owned()
        };

        write!(f, "{} {}", self.status.code(), message)
    }
}