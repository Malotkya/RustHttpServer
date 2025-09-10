/// Http Status
/// 
/// RFC-2616 10
/// https://datatracker.ietf.org/doc/html/rfc2616#section-10
/// 

#[derive(Debug, Clone)]
pub enum HttpStatus {
    //Information Responses
    Continue,
    SwitchingProtocols,
    Processing,

    // OK Responses
    Success,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AltreadyReported,
    ImUsed,
    Ok,

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

impl HttpStatus {
    pub fn as_str(&self)->&'static str {
        match self {
            //Information Responses
            Self::Continue => "CONTINUE",
            Self::SwitchingProtocols => "SWITCHING PROTOCOLS",
            Self::Processing => "PROCESSING",

            // OK Responses
            Self::Success => "SUCCESS",
            Self::Created => "CREATED",
            Self::Accepted => "ACCEPTED",
            Self::NonAuthoritativeInformation => "NON_AUHTORITATIVE INFORMATION",
            Self::NoContent => "NO CONTENT",
            Self::ResetContent => "RESET CONTENT",
            Self::PartialContent => "PARTIAL CONTENT",
            Self::MultiStatus => "MULTI-STATUS",
            Self::AltreadyReported => "ALREADY REPORTED",
            Self::ImUsed => "IM USED",
            Self::Ok => "OK",

            //Redirection Messages
            Self::MultipleChoices => "MULTIPLE CHOICES",
            Self::MovedPermanently => "MOVED PERMANENTLY",
            Self::Found => "FOUND",
            Self::SeeOther => "SEE OTHER",
            Self::NotModified => "NOT MODIFIED",
            Self::UseProxy => "USE PROXY",
            Self::Unused => "UNUSED",
            Self::TemporaryRedirect => "TEMPORARY REDIRECT",
            Self::PermanentRedirect => "PERMANENT REDIRECT",
            Self::Redirect => "REDIRECT",

            //Client Errors
            Self::BadRequest =>"BAD REQUEST",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::PaymentRequired => "PAYMENT REQURED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT FOUND",
            Self::MethodNotAllowed => "METHOD NOT ALLOWED",
            Self::NotAcceptable => "NOT ACCEPTABLE",
            Self::ProxyAuthenticationRequired => "PROXY AUTHENTICATION REQUIRED",
            Self::RequestTimeout => "REQUEST TIMEOUT",
            Self::Conflict => "CONFLICT",
            Self::Gone => "GONE",
            Self::LengthRequired => "LENGTH REQUIRED",
            Self::PreconditionFailed => "PRECONDITION FAILED",
            Self::PayloadTooLarge => "PAYLOAD TOO LARGE",
            Self::UriTooLong => "URI TOO LONG",
            Self::UnsupportedMediaType => "UNSUPPORTED MEDIA TYPE",
            Self::RangeNotSatisfiable => "RANGE NOT SATISFIABLE",
            Self::ExpectationFailed => "EXPECTATION FAILED",
            Self::MisdirectedRequest => "MISDIRECTED REQUEST",
            Self::UnprocessableContent => "UNPROCESSABLE CONTENT",
            Self::Locked => "LOCKED",
            Self::FailedDependency => "FAILED DEPENDENCY",
            Self::TooEarly => "TOO EARLY",
            Self::PreconditionRequired => "PRECONDITION REQUIRED",
            Self::TooManyRequests => "TOO MANY REQUESTS",
            Self::RequestHeaderFieldsTooLarge => "REQUEST HEADER FIELDS TOO LARGE",
            Self::UnableForLeagalReasons =>  "UNABLE FOR LEAGAL REASONS",
            Self::ClientError => "CLIENT ERROR",

            //Server Error
            Self::NotImplemented => "NOT IMPLEMENTED",
            Self::BadGateway => "BAD GATEWAY",
            Self::ServiceUnavailable => "SERVICE UNAVAILABLE",
            Self::GatewayTimeout => "GATEWAY TIMEOUT",
            Self::HttpVersionNotSupported => "HTTP VERSION NOT SUPPORTED",
            Self::VariantAlsoNegotiates => "VARIANT ALSO NEGOTIATES",
            Self::InsufficientStorage => "INSUFFICIENT STORAGE",
            Self::LoopDetected => "LOOP DETECTED",
            Self::NotExtended => "NOT EXTENDED",
            Self::NetworkAuthenticationRequired => "NETWORK AUTHENTICATION REQUIRED",
            Self::InternalServerError => "INTERNAL SERVER ERROR"
        }
    }

    pub fn code(&self)->u16 {
        match self {
            //Information Responses
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,

            // OK Responses
            Self::Success => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AltreadyReported => 208,
            Self::ImUsed => 226,
            Self::Ok => 200,

            //Redirection Messages
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::Unused => 306,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::Redirect => 308, //307??

            //Client Errors
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::PayloadTooLarge => 413,
            Self::UriTooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableContent => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnableForLeagalReasons => 451,
            Self::ClientError => 400,

            //Server Error
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HttpVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
            Self::InternalServerError => 500
        } 
    }
}