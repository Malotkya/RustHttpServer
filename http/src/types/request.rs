use super::{Url, Headers, Method, Version, JsonValue, JsonRef};
use std::boxed::Box;
use std::collections::HashMap;

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

pub trait RequestBody {
    fn body(&mut self) -> Result<&[u8], &'static str>;
    fn data(&mut self) -> Result<BodyData, &'static str>;
}

pub struct RequestBuilder<'body> {
    pub url:Url,
    pub version:Version,
    pub method: Method,
    pub headers: Headers,
    pub buffer: Box<dyn RequestBody +'body>
}

impl<'body> RequestBuilder<'body> {
    pub fn new(url:Url, method:Method, headers:Headers, version:Version, buffer:Box<dyn RequestBody + 'body>) -> Self {
        Self {
            url, method, headers,
            version,
            buffer
        }
    }

    pub fn build<P>(&'body mut self, param:P) -> Request<'body, P> {
        Request {
            builder: self,
            param
        }
    }
}

pub struct Request<'builder, PARAM> {
    builder: &'builder mut RequestBuilder<'builder>,
    pub param: PARAM
}

impl<'b, P> Request<'b, P> {
    pub fn url(&'b self) -> &'b Url {
        &self.builder.url
    }

    pub fn http_version(&'b self) -> &'b Version {
        &self.builder.version
    }

    pub fn method(&'b self) -> &'b Method {
        &self.builder.method
    }

    pub fn headers(&'b self) -> &'b Headers {
        &self.builder.headers
    }

    pub fn body(&'b mut self) -> Result<&'b [u8], &'static str> {
        self.builder.buffer.body()
    }
}