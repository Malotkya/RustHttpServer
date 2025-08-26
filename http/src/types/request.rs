use super::{Url, Headers, Method, Version, JsonValue, JsonRef, HttpError};
use std::collections::HashMap;
use std::io::{BufReader, Read};

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

pub struct RequestBuilder<STREAM> where STREAM: Read {
    pub(crate) url:Url,
    pub(crate) version:Version,
    pub(crate) method: Method,
    pub(crate) headers: Headers,
    buffer: Option<BufReader<STREAM>>,
    body_used:bool
}

impl<S> RequestBuilder<S> where S: Read{
    pub(crate) fn new(url:Url, method:Method, headers:Headers, version:Version, buffer:Option<BufReader<S>> ) -> Self where S: Read {
        Self {
            url, method, headers,
            version, buffer,
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

    pub fn build<'b, P>(&'b mut self, param:P) -> Request<'b, S, P> {
        Request {
            builder: self,
            param
        }
    }

    pub fn error<'b>(&'b mut self, err: HttpError) -> ErrorRequest<'b, S> {
        Request {
            builder: self,
            param: err
        }
    }
}

pub type ErrorRequest<'builder, STREAM> = Request<'builder, STREAM, HttpError>;

pub struct Request<'builder, STREAM, PARAM> where STREAM: Read {
    builder: &'builder mut RequestBuilder<STREAM>,
    pub param: PARAM
}

impl<'b, S, P> Request<'b, S, P> where S: Read{
    pub fn url(&self) -> &Url {
        &self.builder.url
    }

    pub fn version(&self) -> &Version {
        &self.builder.version
    }

    pub fn headers(&self) -> &Headers {
        &self.builder.headers
    }

    pub fn method(&self) -> &Method {
        &self.builder.method
    }

    pub fn body(&mut self) -> Result<Option<&[u8]>, &'static str> {
        self.builder.body()
    }

    pub fn data(&mut self) -> Result<Option<BodyData>, &'static str> {
        match self.builder.body()? {
            Some(_) => todo!("Parse Body Data"),
            None => Ok(None)
        }
    }
}