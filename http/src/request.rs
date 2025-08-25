use crate::{Url, Headers, Method, types::{Version, BodyData}};
use std::boxed::Box;

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

#[allow(dead_code)]
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

#[allow(dead_code)]
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