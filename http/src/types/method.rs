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

#[allow(dead_code)]
impl Method {
    pub fn from(str: &str)->Option<Method> {
        match str.to_ascii_uppercase().as_str() {
            "OPTIONS" =>
                Some(Self::OPTIONS),
            "GET" =>
                Some(Self::GET),
            "HEAD" => 
                Some(Self::HEAD),
            "POST" =>
                Some(Self::POST),
            "PUT" =>
                Some(Self::PUT),
            "DELETE" =>
                Some(Self::DELETE),
            "TRACE" =>
                Some(Self::TRACE),
            "CONNECT" =>
                Some(Self::CONNECT),
            "PATCH" => {
                Some(Self::PATCH)
            },
            _ => None
        }
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