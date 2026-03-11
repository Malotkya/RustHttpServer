#![feature(str_from_raw_parts)]

pub mod error;
pub mod headers;
pub mod method;
pub mod path;
pub mod protocol;
pub mod request;
pub mod response;
pub mod server;
pub mod status;
pub mod url;

pub mod result {
    use super::error::ValidHttpError;

    pub type Result<T> = std::result::Result<T, Box<dyn ValidHttpError>>;
}

pub mod version {
    pub struct Version {
        pub major: u8,
        pub minor: u8
    }

    impl ToString for Version {
        fn to_string(&self) -> String {
            format!("HTTP/{}.{}", self.major, self.minor)
        }
    }
}

pub mod router {
    use crate::{
        //server::,
        request::{Request, RequestBuilder},
        response::Response,
        error::HttpError,
        result::Result
    };

    pub trait Layer<Param> {
        fn new() -> Self;
        fn match_path(&self, pathname:&str) -> Option<Param>;
        fn handler(&self, request: Request<Param>) -> impl Future<Output = Result<Response>>;
    }

    pub trait Router<Param>: Layer<Param> {

        #[allow(async_fn_in_trait)]
        async fn handle(&self, req:&mut RequestBuilder<async_lib::net::TcpStream>) -> std::result::Result<Option<Response>, HttpError> {
            match self.match_path(&req.url.pathname()) {
                Some(param) => match self.handler(req.build(param)).await {
                    Ok(resp) => Ok(Some(resp)),
                    Err(err) => Err(err.err())
                }
                None => Ok(None)
            }
        }
    }
}

