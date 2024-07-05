use std::fmt;

#[derive(Debug, Clone)]
pub struct HttpError{
    message: String,
    kind: HttpErrorKind
}

impl HttpError{
    pub fn new<E:fmt::Display>(kind: HttpErrorKind, error: E)-> HttpError{
        Self{ kind, message: format!("{}", error) }
    }

    pub fn message(&self)->&str {
        if self.message.len() == 0 {
            return self.kind.as_str();
        }

        &self.message
    }

    pub fn code(&self)->u16 {
        self.kind.code()
    }

    pub fn kind(&self)->&HttpErrorKind {
        &self.kind
    }
}

#[derive(Debug, Clone)]
pub enum HttpErrorKind {
    //Information Responses
    Continue,
    SwitchingProtocols,
    Processing,
    Unknown,

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


impl HttpErrorKind {
    pub fn from(code: u16)->HttpErrorKind {
        //Information Responses
        if code == 100 {
            return HttpErrorKind::Continue;
        } else if code == 101 {
            return HttpErrorKind::SwitchingProtocols;
        } else if code == 102 {
            return  HttpErrorKind::Processing;
        } else if code < 200 {
            return HttpErrorKind::Unknown;

        // OK Responses    
        }else if code == 200 {
            return HttpErrorKind::Success;
        } else if code == 201 {
            return HttpErrorKind::Created;
        } else if code == 202 {
            return HttpErrorKind::Accepted;
        } else if code == 203 {
            return HttpErrorKind::NonAuthoritativeInformation;
        } else if code == 204 {
            return HttpErrorKind::NoContent;
        } else if code == 205 {
            return HttpErrorKind::ResetContent;
        } else if code == 206 {
            return HttpErrorKind::PartialContent;
        } else if code == 207 {
            return HttpErrorKind::MultiStatus;
        } else if code == 208 {
            return HttpErrorKind::AltreadyReported;
        } else if code == 226 {
            return  HttpErrorKind::ImUsed;
        } else if code < 300 {
            return HttpErrorKind::Ok;

        //Redirection Messages
        } else if code == 300 {
            return HttpErrorKind::MultipleChoices;
        } else if code == 301 {
            return HttpErrorKind::MovedPermanently;
        } else if code == 302 {
            return HttpErrorKind::Found;
        } else if code == 303 {
            return HttpErrorKind::SeeOther;
        } else if code == 304 {
            return HttpErrorKind::NotModified;
        } else if code == 305 {
            return HttpErrorKind::UseProxy;
        } else if code == 306 {
            return HttpErrorKind::Unused;
        } else if code == 307 {
            return HttpErrorKind::TemporaryRedirect;
        } else if code == 308 {
            return HttpErrorKind::PermanentRedirect;
        } else if code < 400 {
            return HttpErrorKind::Redirect;

        //Client Errors
        } else if code == 400 {
            return HttpErrorKind::BadRequest;
        } else if code == 401 {
            return HttpErrorKind::Unauthorized;
        } else if code == 402 {
            return HttpErrorKind::PaymentRequired;
        } else if code == 403 {
            return HttpErrorKind::Forbidden;
        } else if code == 404 {
            return HttpErrorKind::NotFound;
        } else if code == 405 {
            return HttpErrorKind::MethodNotAllowed;
        } else if code == 406 {
            return HttpErrorKind::NotAcceptable;
        } else if code == 407 {
            return HttpErrorKind::ProxyAuthenticationRequired;
        } else if code == 408 {
            return HttpErrorKind::RequestTimeout;
        } else if code == 409 {
            return HttpErrorKind::Conflict;
        } else if code == 410 {
            return HttpErrorKind::Gone;
        } else if code == 411 {
            return HttpErrorKind::LengthRequired;
        } else if code == 412 {
            return HttpErrorKind::PreconditionFailed;
        } else if code == 413 {
            return HttpErrorKind::PayloadTooLarge;
        } else if code == 414 {
            return HttpErrorKind::UriTooLong;
        } else if code == 415 {
            return HttpErrorKind::UnsupportedMediaType;
        } else if code == 416 {
            return HttpErrorKind::RangeNotSatisfiable;
        } else if code == 417 {
            return HttpErrorKind::ExpectationFailed;
        } else if code == 421 {
            return HttpErrorKind::MisdirectedRequest;
        } else if code == 422 {
            return HttpErrorKind::UnprocessableContent;
        } else if code == 423 {
            return HttpErrorKind::Locked;
        } else if code == 424 {
            return HttpErrorKind::FailedDependency;
        } else if code == 425 {
            return HttpErrorKind::TooEarly;
        } else if code == 428 {
            return HttpErrorKind::PreconditionRequired;
        } else if code == 429 {
            return HttpErrorKind::TooManyRequests;
        } else if code == 431 {
            return HttpErrorKind::RequestHeaderFieldsTooLarge;
        } else if code == 451 {
            return HttpErrorKind::UnableForLeagalReasons;
        } else if code < 500 {
            return HttpErrorKind::ClientError;

            //Server Error
        } else if code == 501 {
            return HttpErrorKind::NotImplemented;
        } else if code == 502 {
            return HttpErrorKind::BadGateway;
        } else if code == 503 {
            return HttpErrorKind::ServiceUnavailable;
        } else if code == 504 {
            return HttpErrorKind::GatewayTimeout;
        } else if code == 505 {
            return HttpErrorKind::HttpVersionNotSupported;
        } else if code == 506 {
            return HttpErrorKind::VariantAlsoNegotiates;
        } else if code == 507 {
            return HttpErrorKind::InsufficientStorage;
        } else if code == 508 {
            return HttpErrorKind::LoopDetected;
        } else if code == 510 {
            return HttpErrorKind::NotExtended;
        } else if code == 511 {
            return HttpErrorKind::NetworkAuthenticationRequired;
        } else {
            return HttpErrorKind::InternalServerError;
        }
    }

    pub fn as_str(&self)->&str {
        match self {
            //Information Responses
            HttpErrorKind::Continue => "CONTINUE",
            HttpErrorKind::SwitchingProtocols => "SWITCHING PROTOCOLS",
            HttpErrorKind::Processing => "PROCESSING",
            HttpErrorKind::Unknown => "UNKOWN",

            // OK Responses
            HttpErrorKind::Success => "SUCCESS",
            HttpErrorKind::Created => "CREATED",
            HttpErrorKind::Accepted => "ACCEPTED",
            HttpErrorKind::NonAuthoritativeInformation => "NON_AUHTORITATIVE INFORMATION",
            HttpErrorKind::NoContent => "NO CONTENT",
            HttpErrorKind::ResetContent => "RESET CONTENT",
            HttpErrorKind::PartialContent => "PARTIAL CONTENT",
            HttpErrorKind::MultiStatus => "MULTI-STATUS",
            HttpErrorKind::AltreadyReported => "ALREADY REPORTED",
            HttpErrorKind::ImUsed => "IM USED",
            HttpErrorKind::Ok => "OK",

            //Redirection Messages
            HttpErrorKind::MultipleChoices => "MULTIPLE CHOICES",
            HttpErrorKind::MovedPermanently => "MOVED PERMANENTLY",
            HttpErrorKind::Found => "FOUND",
            HttpErrorKind::SeeOther => "SEE OTHER",
            HttpErrorKind::NotModified => "NOT MODIFIED",
            HttpErrorKind::UseProxy => "USE PROXY",
            HttpErrorKind::Unused => "UNUSED",
            HttpErrorKind::TemporaryRedirect => "TEMPORARY REDIRECT",
            HttpErrorKind::PermanentRedirect => "PERMANENT REDIRECT",
            HttpErrorKind::Redirect => "REDIRECT",

            //Client Errors
            HttpErrorKind::BadRequest =>"BAD REQUEST",
            HttpErrorKind::Unauthorized => "UNAUTHORIZED",
            HttpErrorKind::PaymentRequired => "PAYMENT REQURED",
            HttpErrorKind::Forbidden => "FORBIDDEN",
            HttpErrorKind::NotFound => "NOT FOUND",
            HttpErrorKind::MethodNotAllowed => "METHOD NOT ALLOWED",
            HttpErrorKind::NotAcceptable => "NOT ACCEPTABLE",
            HttpErrorKind::ProxyAuthenticationRequired => "PROXY AUTHENTICATION REQUIRED",
            HttpErrorKind::RequestTimeout => "REQUEST TIMEOUT",
            HttpErrorKind::Conflict => "CONFLICT",
            HttpErrorKind::Gone => "GONE",
            HttpErrorKind::LengthRequired => "LENGTH REQUIRED",
            HttpErrorKind::PreconditionFailed => "PRECONDITION FAILED",
            HttpErrorKind::PayloadTooLarge => "PAYLOAD TOO LARGE",
            HttpErrorKind::UriTooLong => "URI TOO LONG",
            HttpErrorKind::UnsupportedMediaType => "UNSUPPORTED MEDIA TYPE",
            HttpErrorKind::RangeNotSatisfiable => "RANGE NOT SATISFIABLE",
            HttpErrorKind::ExpectationFailed => "EXPECTATION FAILED",
            HttpErrorKind::MisdirectedRequest => "MISDIRECTED REQUEST",
            HttpErrorKind::UnprocessableContent => "UNPROCESSABLE CONTENT",
            HttpErrorKind::Locked => "LOCKED",
            HttpErrorKind::FailedDependency => "FAILED DEPENDENCY",
            HttpErrorKind::TooEarly => "TOO EARLY",
            HttpErrorKind::PreconditionRequired => "PRECONDITION REQUIRED",
            HttpErrorKind::TooManyRequests => "TOO MANY REQUESTS",
            HttpErrorKind::RequestHeaderFieldsTooLarge => "REQUEST HEADER FIELDS TOO LARGE",
            HttpErrorKind::UnableForLeagalReasons =>  "UNABLE FOR LEAGAL REASONS",
            HttpErrorKind::ClientError => "CLIENT ERROR",

            //Server Error
            HttpErrorKind::NotImplemented => "NOT IMPLEMENTED",
            HttpErrorKind::BadGateway => "BAD GATEWAY",
            HttpErrorKind::ServiceUnavailable => "SERVICE UNAVAILABLE",
            HttpErrorKind::GatewayTimeout => "GATEWAY TIMEOUT",
            HttpErrorKind::HttpVersionNotSupported => "HTTP VERSION NOT SUPPORTED",
            HttpErrorKind::VariantAlsoNegotiates => "VARIANT ALSO NEGOTIATES",
            HttpErrorKind::InsufficientStorage => "INSUFFICIENT STORAGE",
            HttpErrorKind::LoopDetected => "LOOP DETECTED",
            HttpErrorKind::NotExtended => "NOT EXTENDED",
            HttpErrorKind::NetworkAuthenticationRequired => "NETWORK AUTHENTICATION REQUIRED",
            HttpErrorKind::InternalServerError => "INTERNAL SERVER ERROR"
        }
    }

    pub fn code(&self)->u16 {
        match self {
            //Information Responses
            HttpErrorKind::Continue => 100,
            HttpErrorKind::SwitchingProtocols => 101,
            HttpErrorKind::Processing => 102,
            HttpErrorKind::Unknown => 500,

            // OK Responses
            HttpErrorKind::Success => 200,
            HttpErrorKind::Created => 201,
            HttpErrorKind::Accepted => 202,
            HttpErrorKind::NonAuthoritativeInformation => 203,
            HttpErrorKind::NoContent => 204,
            HttpErrorKind::ResetContent => 205,
            HttpErrorKind::PartialContent => 206,
            HttpErrorKind::MultiStatus => 207,
            HttpErrorKind::AltreadyReported => 208,
            HttpErrorKind::ImUsed => 226,
            HttpErrorKind::Ok => 200,

            //Redirection Messages
            HttpErrorKind::MultipleChoices => 300,
            HttpErrorKind::MovedPermanently => 301,
            HttpErrorKind::Found => 302,
            HttpErrorKind::SeeOther => 303,
            HttpErrorKind::NotModified => 304,
            HttpErrorKind::UseProxy => 305,
            HttpErrorKind::Unused => 306,
            HttpErrorKind::TemporaryRedirect => 307,
            HttpErrorKind::PermanentRedirect => 308,
            HttpErrorKind::Redirect => 308, //307??

            //Client Errors
            HttpErrorKind::BadRequest => 400,
            HttpErrorKind::Unauthorized => 401,
            HttpErrorKind::PaymentRequired => 402,
            HttpErrorKind::Forbidden => 403,
            HttpErrorKind::NotFound => 404,
            HttpErrorKind::MethodNotAllowed => 405,
            HttpErrorKind::NotAcceptable => 406,
            HttpErrorKind::ProxyAuthenticationRequired => 407,
            HttpErrorKind::RequestTimeout => 408,
            HttpErrorKind::Conflict => 409,
            HttpErrorKind::Gone => 410,
            HttpErrorKind::LengthRequired => 411,
            HttpErrorKind::PreconditionFailed => 412,
            HttpErrorKind::PayloadTooLarge => 413,
            HttpErrorKind::UriTooLong => 414,
            HttpErrorKind::UnsupportedMediaType => 415,
            HttpErrorKind::RangeNotSatisfiable => 416,
            HttpErrorKind::ExpectationFailed => 417,
            HttpErrorKind::MisdirectedRequest => 421,
            HttpErrorKind::UnprocessableContent => 422,
            HttpErrorKind::Locked => 423,
            HttpErrorKind::FailedDependency => 424,
            HttpErrorKind::TooEarly => 425,
            HttpErrorKind::PreconditionRequired => 428,
            HttpErrorKind::TooManyRequests => 429,
            HttpErrorKind::RequestHeaderFieldsTooLarge => 431,
            HttpErrorKind::UnableForLeagalReasons => 451,
            HttpErrorKind::ClientError => 400,

            //Server Error
            HttpErrorKind::NotImplemented => 501,
            HttpErrorKind::BadGateway => 502,
            HttpErrorKind::ServiceUnavailable => 503,
            HttpErrorKind::GatewayTimeout => 504,
            HttpErrorKind::HttpVersionNotSupported => 505,
            HttpErrorKind::VariantAlsoNegotiates => 506,
            HttpErrorKind::InsufficientStorage => 507,
            HttpErrorKind::LoopDetected => 508,
            HttpErrorKind::NotExtended => 510,
            HttpErrorKind::NetworkAuthenticationRequired => 511,
            HttpErrorKind::InternalServerError => 500
        } 
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}