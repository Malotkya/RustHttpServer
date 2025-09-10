use super::{Url, Headers, Method, Version, JsonValue, JsonRef, HttpError};
use std::{
    collections::HashMap,
    io::BufReader,
    fmt,
    net::TcpStream
};

pub enum BodyDataType {
    File(FileData),
    Value(JsonValue)
}

impl BodyDataType {
    pub fn new_file(filename:&str, mimetype:&str, data:&[u8]) -> Self {
        Self::File(
            FileData {
                name: String::from(filename),
                mimetype: String::from(mimetype),
                data: data.into()
            }
        )
    }

    pub fn new(data:&str) -> Self {
        Self::Value(
            JsonValue::String(String::from(data))
        )
    }

    pub fn value<'a>(&'a self) -> Option<JsonRef<'a>> {
        match self {
            Self::Value(data) => Some(data.into()),
            Self::File(_) => None
        }
    }

    pub fn file<'a>(&'a self) -> Option<&'a FileData> {
        match self {
            Self::File(data) => Some(data),
            Self::Value(_) => None
        }
    }
}

pub type BodyData = HashMap<String, BodyDataType>;

pub struct FileData {
    pub name: String,
    pub mimetype: String,
    pub data: Vec<u8>
}

pub struct RequestBuilder<STREAM: std::io::Read> {
    pub url:Url,
    pub version:Version,
    pub method: Method,
    pub headers: Headers,
    buffer: Option<BufReader<STREAM>>,
    body_used:bool
}

impl<S: std::io::Read> RequestBuilder<S> {
    pub fn new(url:Url, method:Method, headers:Headers, version:Version, stream:Option<BufReader<S>>) -> Self{
        Self {
            url, method, headers,
            version,
            buffer: stream,
            body_used: false
        }
    }

    pub fn body(&mut self) -> Result<Option<&[u8]>, &'static str> {
        if self.body_used {
            Err("Request Body is already used!")
        } else {
            self.body_used = false;
            match &self.buffer {
                Some(reader) => Ok(Some(reader.buffer())),
                None => Ok(None)
            }
        }
    }
}

impl RequestBuilder<TcpStream> {
    pub fn build<P>(&mut self, param:P) -> Request<P> {
        Request {
            builder: std::ptr::from_mut(self),
            param
        }
    }

    pub fn error(&mut self, err: HttpError) -> ErrorRequest {
        Request {
            builder: std::ptr::from_mut(self),
            param: err
        }
    }
}

impl<S: std::io::Read> fmt::Debug for RequestBuilder<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.method.to_str(), self.url.pathname())
    }
}

pub type ErrorRequest = Request<HttpError>;

pub struct Request<PARAM> {
    builder: *mut RequestBuilder<TcpStream>,
    pub param: PARAM
}

impl<P> Request<P>{
    pub fn url(&self) -> &Url {
        unsafe{ &(*self.builder).url }
    }

    pub fn version(&self) -> &Version {
        unsafe{ &(*self.builder).version }
    }

    pub fn headers(&self) -> &Headers {
        unsafe{ &(*self.builder).headers }
    }

    pub fn method(&self) -> &Method {
        unsafe{ &(*self.builder).method }
    }

    pub fn body(&mut self) -> Result<Option<&[u8]>, &'static str> {
        unsafe{ (*self.builder).body() }
    }

    pub fn data(&mut self) -> Result<Option<BodyData>, &'static str> {
        match self.body()? {
            Some(_) => todo!("Parse Body Data"),
            None => Ok(None)
        }
    }
}