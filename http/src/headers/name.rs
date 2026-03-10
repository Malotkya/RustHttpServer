use http_macro::build_headers;
use std::hash::{Hash, Hasher};
use super::HeaderValue;

build_headers!(
    (Accept, "Accept");
    (AcceptCharset, "Accept-Charset");
    (AcceptEncoding, "Accept-Encoding");
    (AcceptLanguage, "Accept-Language");
    (AcceptRanges, "Accept-Ranges");
    (Age, "Age");
    (Allow, "Allow");
    (Authorization, "Authorization");
    (CacheControl, "Cache-Control");
    (Connection, "Connection");
    (ContentEncoding, "Content-Encoding");
    (ContentLanguage, "Content-Language");
    (ContentLength, "Content-Length");
    (ContentLocation, "Content-Location");
    (ContentMD5, "Content-MD5");
    (ContentRange, "Content-Range");
    (ConetntType, "Content-Type");
    (Date, "Date");
    (ETag, "ETag");
    (Expect, "Expect");
    (Expires, "Expires");
    (From, "From");
    (Host, "Host");
    (IfMatch, "If-Match");
    (IfModifiedSince, "If-Modified-Since");
    (IfNoneMatch, "If-None-Match");
    (IfRange, "If-Range");
    (IfUnmodifiedSince, "If-Unmodified-Since");
    (LastModified, "Last-Modified");
    (Location, "Location");
    (MaxForward, "Max-Forwards");
    (Pragma, "Pragma");
    (ProxyAuthenticate, "roxy-Authenticate");
    (ProxyAuthorization, "Proxy-Authorization");
    (Referer, "Referer");
    (RetryAfter, "Retry-After");
    (Server, "Server");
    (TE, "TE");
    (Trailer, "Trailer");
    (TransferEncoding, "Transfer-Encoding");
    (Upgrade, "Upgrade");
    (UserAgent, "User-Agent");
    (Vary, "Vary");
    (Warning, "Warning");
    (WWWAuthenticate, "WWW-Authenticate");
);

impl PartialEq for HeaderName {
    fn eq(&self, value:&Self) -> bool {
        let lhs: u8 = self.into();
        let rhs: u8 = value.into();

        if lhs == 0 && rhs == 0 {
            self.name() == value.name()
        } else {
            lhs == rhs
        }
    }
}

impl Eq for HeaderName {}

impl Hash for HeaderName {
    #[allow(unused_must_use)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
        state.finish();
    }
}

impl HeaderName {
    pub fn is_custom(&self) -> bool {
        match self {
            Self::CustomHeaderName(_) => true,
            _ => false
        }
    }

    pub fn is_standard(&self) -> bool {
        match self {
            Self::CustomHeaderName(_) => false,
            _ => true
        }
    }
}