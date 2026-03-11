#![feature(str_from_raw_parts)]
#![allow(unused_imports)]

use http_core::{Request, RequestBuilder};
use async_lib::{
    io::AsyncRead,
    net::TcpStream
};

pub use http_core::{
    HttpError, HttpErrorKind, Result
};

mod protocol;
pub(crate) use protocol::*;
mod server;

mod html {
    pub use html::*;
}

pub trait Layer<Param> {
    fn new() -> Self;
    fn match_path(&self, pathname:&str) -> Option<Param>;
    fn handler(&self, request: Request<Param>) -> impl Future<Output = http_core::Result>;
}

pub trait Router<Param>: Layer<Param> {

    #[allow(async_fn_in_trait)]
    async fn handle(&self, req:&mut http_core::RequestBuilder<async_lib::net::TcpStream>) -> std::result::Result<Option<http_core::Response>, http_core::HttpError> {
        match self.match_path(&req.url.pathname()) {
            Some(param) => self.handler(req.build(param)).await.map(|resp|Some(resp)),
            None => Ok(None)
        }
    }
}

impl<P, R:Layer<P>> Router<P> for R {}

pub mod builder {
    pub use http_core::RequestBuilder;
    pub use super::server::*;
    pub use http_macro::*;
}

pub mod json {
    pub use util::json::{
        JsonError, JsonRef, JsonValue
    };
}

pub mod types {
    pub use http_core::{
        HttpHeader, Headers, HeaderName, HeaderValue, Version,
        Method, Path, Request, ErrorRequest, Response, HttpStatus, Url
    };
}

pub mod async_net {
    pub use async_lib::net::*;
}

pub mod async_io {
    pub use async_lib::io::*;
}

pub mod async_fs {
    pub use async_lib::fs::*;
}

pub mod event {
    pub use async_lib::{
        EventEmitter, EventEmitterWrapper
    };
}

pub mod promise {
    pub use async_lib::{
        promise, Promise,
    };
}

pub mod executor {
    pub use async_lib::executor::*;
}

pub(crate) fn log(req: &RequestBuilder<impl AsyncRead>, resp: &types::Response) {
    println!("{:?} {:?}", req, resp);
}
