/// Http/0.9 Request Format:
/// 
/// GET [PATH]
/// 
use http::{Headers, Method, Url, types::Version};
use http::request::{RequestBuilder, RequestBody};
use crate::http1::request::BuildError;

struct EmptyBody;

impl RequestBody for EmptyBody {
    fn body(&self) -> Result<&[u8], &'static str> {
        Err("No body is abailable in Http0 requests!")
    }
}

pub fn build(port:u16, method: Method, path: &str) -> Result<RequestBuilder<EmptyBody>, BuildError> {
    if method != Method::GET {
        return Err(BuildError::OnlyGetMethod(method))
    }

    Ok(RequestBuilder::new(
        Url::empty(port, path),
        method,
        Headers::new(),
        Version{major: 0, minor: 9},
        EmptyBody
    ))
}