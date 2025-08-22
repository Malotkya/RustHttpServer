/// Http/0.9 Request Format:
/// 
/// GET [PATH]
/// 
use std::fmt;
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
    OnlyGetMethod,
    OnlyAbsolutePath,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OnlyAbsolutePath => write!(f, "Only absolute paths are allowed in http/0.9 requests!"),
            Self::OnlyGetMethod => write!(f, "Only Get methods are allowed in http/0.9 requests!"),
        }
    }
}

pub fn build<'a>(port:u16, method: Method, path:Uri) -> Result<RequestBuilder<'a>, BuildError> {
    if method != Method::GET {
        return Err(BuildError::OnlyGetMethod)
    }

    let path = match path.absolute_path() {
        Ok(str) => str,
        Err(_) => return Err(BuildError::OnlyAbsolutePath)
    };

    Ok(RequestBuilder::new(
        Url::empty(port, &path),
        method,
        Headers::new(),
        Version{major: 0, minor: 9},
        Box::new(EmptyBody)
    ))
}