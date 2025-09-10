#![feature(str_from_raw_parts)]
use http_types::{RequestBuilder, Response, HttpError};
use async_lib::{
    io::AsyncRead,
    net::TcpStream
};

mod protocol;
pub use protocol::*;
pub mod server;
pub use server::*;

pub trait Router {
    fn handle(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = std::result::Result<Option<Response>, HttpError>>;
}

pub(crate) fn log(req: &RequestBuilder<impl AsyncRead>, resp: &Response) {
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