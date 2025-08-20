/// Http/0.9 Request Format:
/// 
/// GET [PATH]
/// 
use http::{Headers, Method, Url, types::Version};
use http::request::{RequestBuilder, RequestBody};
use crate::http1::{types::Uri};

struct EmptyBody;

impl RequestBody for EmptyBody {
    fn body(&mut self) -> Result<&[u8], &'static str> {
        Err("No body is abailable in Http0 requests!")
    }
}

pub enum BuildError {
    OnlyGetMethod(Method),
    OnlyAbsolutePath(String),
}

pub fn build(port:u16, method: Method, path:Uri) -> Result<RequestBuilder<EmptyBody>, BuildError> {
    if method != Method::GET {
        return Err(BuildError::OnlyGetMethod(method))
    }

    let path = match path.absolute_path() {
        Ok(str) => str,
        Err(e) => return Err(BuildError::OnlyAbsolutePath(e))
    };

    Ok(RequestBuilder::new(
        Url::empty(port, &path),
        method,
        Headers::new(),
        Version{major: 0, minor: 9},
        EmptyBody
    ))
}