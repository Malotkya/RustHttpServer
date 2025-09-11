#![feature(str_from_raw_parts)]
use http_types::{RequestBuilder};
use async_lib::{
    io::AsyncRead,
    net::TcpStream
};

pub use http_types::{
    HttpError, HttpErrorKind, Result
};

mod protocol;
pub(crate) use protocol::*;
mod server;


pub trait Router {
    fn handle(&self, req:&mut RequestBuilder<TcpStream>)
        -> impl Future<Output = std::result::Result<Option<types::Response>, HttpError>>;
}

pub mod builder {
    pub use http_types::RequestBuilder;
    pub use super::server::*;
    pub use http_macro::*;
}

pub mod json {
    pub use http_types::{
        Json, JsonError, JsonRef, JsonValue
    };
}

pub mod types {
    pub use http_types::{
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

pub fn load_settings(_path:&str) -> (Option<u16>, Option<String>){
    todo!("Ability to load from settings file.") 
}

pub fn get_arguments() -> (Option<u16>, Option<String>, Option<String>) {
    let mut port = None;
    let mut hostname = None;
    let mut config_file = None;

    for input in std::env::args() {
        if let Some(index) = input.find("=") {
            let key = &input[..index];
            let value = &input[index+1..];

            match key.to_ascii_lowercase().as_str() {
                "port" => {
                    port = Some(value.parse().unwrap())
                },
                "hostname" => {
                    hostname = Some(value.to_owned())
                },
                "config" => {
                    config_file = Some(value.to_owned())
                },
                key => {
                    panic!("Unknown command line argument: {key}!")
                }
            }
        }
    }

    (port, hostname, config_file)
}