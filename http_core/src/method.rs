/// Http Method
/// 
/// RFC-2616 5.1.1
/// https://datatracker.ietf.org/doc/html/rfc2616#section-5
/// 

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
    PATCH
}

fn valid_methods_from_string(str: &str)->Option<Method> {
    match str.to_ascii_uppercase().as_str() {
        "OPTIONS" =>
            Some(Method::OPTIONS),
        "GET" =>
            Some(Method::GET),
        "HEAD" => 
            Some(Method::HEAD),
        "POST" =>
            Some(Method::POST),
        "PUT" =>
            Some(Method::PUT),
        "DELETE" =>
            Some(Method::DELETE),
        "TRACE" =>
            Some(Method::TRACE),
        "CONNECT" =>
            Some(Method::CONNECT),
        "PATCH" => {
            Some(Method::PATCH)
        },
        _ => None
    }
}

#[allow(dead_code)]
impl Method {
    pub fn from(str: &str)->Option<Method> {
        let value = str.trim().to_ascii_uppercase();

        valid_methods_from_string(&value)
            .or(valid_methods_from_string(&value[1..])) //Attempt to remove occasional garbage byte.
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::OPTIONS => "OPTIONS",
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::TRACE => "TRACE",
            Self::CONNECT => "CONNECT",
            Self::PATCH => "PATCH"
        }
    }
}