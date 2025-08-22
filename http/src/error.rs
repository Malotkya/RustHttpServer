use std::fmt;

use crate::HttpStatus;

#[derive(Clone)]
pub struct HttpError{
    message: String,
    kind: HttpErrorKind
}

impl HttpError {
    pub fn new(kind: HttpErrorKind, message:&str) -> Self {
        Self{
            kind,
            message: String::from(message)
        }
    }
}

#[derive(Debug, Clone)]
pub enum HttpErrorKind {
    //Redirection Messages
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    Unused,
    TemporaryRedirect,
    PermanentRedirect,
    Redirect,

    //Client Errors
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnableForLeagalReasons,
    ClientError,

    //Server Error
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
    InternalServerError
}

impl HttpErrorKind {
    pub fn as_str(&self)->&'static str {
        match self {
            //Redirection Messages
            Self::MultipleChoices => "Multiple Choices",
            Self::MovedPermanently => "Moved Permanently",
            Self::Found => "Found",
            Self::SeeOther => "See Other",
            Self::NotModified => "Not Modified",
            Self::UseProxy => "Use Proxy",
            Self::Unused => "Unused",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::PermanentRedirect => "Permanent Redirect",
            Self::Redirect => "Redirect",

            //Client Errors
            Self::BadRequest =>"Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "Payment Requred",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::NotAcceptable => "Not Acceptable",
            Self::ProxyAuthenticationRequired => "Proxy Authentication Required",
            Self::RequestTimeout => "Request Timeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "Length Required",
            Self::PreconditionFailed => "Precondition Failed",
            Self::PayloadTooLarge => "Payload Too Large",
            Self::UriTooLong => "Uri Too Long",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::RangeNotSatisfiable => "Range Not Satisfiable",
            Self::ExpectationFailed => "Expectation Failed",
            Self::MisdirectedRequest => "Misdirected Request",
            Self::UnprocessableContent => "Unprocessable Content",
            Self::Locked => "Locked",
            Self::FailedDependency => "Failed Dependency",
            Self::TooEarly => "Too Early",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::UnableForLeagalReasons =>  "Unable For Leagal Reasons",
            Self::ClientError => "Client Error",

            //Server Error
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HttpVersionNotSupported => "Http Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthenticationRequired => "Network Authentication Required",
            Self::InternalServerError => "Internal Server Error"
        }
    }
}

impl Into<HttpStatus> for HttpErrorKind {
    fn into(self) -> HttpStatus {
        match self {
            //Redirection Messages
            Self::MultipleChoices => HttpStatus::MultipleChoices,
            Self::MovedPermanently => HttpStatus::MovedPermanently,
            Self::Found => HttpStatus::Found,
            Self::SeeOther => HttpStatus::SeeOther,
            Self::NotModified => HttpStatus::NotModified,
            Self::UseProxy => HttpStatus::UseProxy,
            Self::Unused => HttpStatus::Unused,
            Self::TemporaryRedirect => HttpStatus::TemporaryRedirect,
            Self::PermanentRedirect => HttpStatus::PermanentRedirect,
            Self::Redirect => HttpStatus::Redirect,

            //Client Errors
            Self::BadRequest => HttpStatus::BadRequest,
            Self::Unauthorized => HttpStatus::Unauthorized,
            Self::PaymentRequired => HttpStatus::PaymentRequired,
            Self::Forbidden => HttpStatus::Forbidden,
            Self::NotFound => HttpStatus::NotFound,
            Self::MethodNotAllowed => HttpStatus::MethodNotAllowed,
            Self::NotAcceptable => HttpStatus::NotAcceptable,
            Self::ProxyAuthenticationRequired => HttpStatus::ProxyAuthenticationRequired,
            Self::RequestTimeout => HttpStatus::RequestTimeout,
            Self::Conflict => HttpStatus::Conflict,
            Self::Gone => HttpStatus::Gone,
            Self::LengthRequired => HttpStatus::LengthRequired,
            Self::PreconditionFailed => HttpStatus::PreconditionFailed,
            Self::PayloadTooLarge => HttpStatus::PayloadTooLarge,
            Self::UriTooLong => HttpStatus::UriTooLong,
            Self::UnsupportedMediaType => HttpStatus::UnsupportedMediaType,
            Self::RangeNotSatisfiable => HttpStatus::RangeNotSatisfiable,
            Self::ExpectationFailed => HttpStatus::ExpectationFailed,
            Self::MisdirectedRequest => HttpStatus::MisdirectedRequest,
            Self::UnprocessableContent => HttpStatus::UnprocessableContent,
            Self::Locked => HttpStatus::Locked,
            Self::FailedDependency => HttpStatus::FailedDependency,
            Self::TooEarly => HttpStatus::TooEarly,
            Self::PreconditionRequired => HttpStatus::PreconditionRequired,
            Self::TooManyRequests => HttpStatus::TooManyRequests,
            Self::RequestHeaderFieldsTooLarge => HttpStatus::RequestHeaderFieldsTooLarge,
            Self::UnableForLeagalReasons =>  HttpStatus::UnableForLeagalReasons,
            Self::ClientError => HttpStatus::ClientError,

            //Server Error
            Self::NotImplemented => HttpStatus::NotImplemented,
            Self::BadGateway => HttpStatus::BadGateway,
            Self::ServiceUnavailable => HttpStatus::ServiceUnavailable,
            Self::GatewayTimeout => HttpStatus::GatewayTimeout,
            Self::HttpVersionNotSupported => HttpStatus::HttpVersionNotSupported,
            Self::VariantAlsoNegotiates => HttpStatus::VariantAlsoNegotiates,
            Self::InsufficientStorage => HttpStatus::InsufficientStorage,
            Self::LoopDetected => HttpStatus::LoopDetected,
            Self::NotExtended => HttpStatus::NotExtended,
            Self::NetworkAuthenticationRequired => HttpStatus::NetworkAuthenticationRequired,
            Self::InternalServerError => HttpStatus::InternalServerError
        }
    }
}

impl Into<HttpError> for HttpErrorKind {
    fn into(self) -> HttpError {
        HttpError::new(self.clone(), self.as_str())
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}: {}",
            Into::<HttpStatus>::into(self.kind.clone()).code(),
            self.message
        )
    }
}

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "HTTP Error({}): {}",
            Into::<HttpStatus>::into(self.kind.clone()).code(),
            self.message
        )
    }
}