use crate::{Url, Headers, Method, types::Version};

pub trait RequestBody: Sized {
    fn body(&self) -> Result<&[u8], &'static str>;
}

pub struct RequestBuilder<BODY>
    where BODY: RequestBody {
    pub url:Url,
    pub version:Version,
    pub method: Method,
    pub headers: Headers,
    pub buffer: BODY,
}

#[allow(dead_code)]
impl<B> RequestBuilder<B>
    where B: RequestBody {
    pub fn new(url:Url, method:Method, headers:Headers, version:Version, buffer:B) -> Self {
        Self {
            url, method, headers,
            version, buffer
        }
    }

    pub fn build<'a, P>(&'a mut self, param:P) -> Request<'a, P, B> {
        Request {
            builder: self,
            param
        }
    }
}

pub struct Request<'builder, PARAM, BODY> 
    where BODY: RequestBody {
    builder: &'builder mut RequestBuilder<BODY>,
    pub param: PARAM
}

#[allow(dead_code)]
impl<'b, P, B> Request<'b, P, B>
    where B: RequestBody {
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