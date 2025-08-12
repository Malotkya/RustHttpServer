use std::collections::HashMap;
use std::net::TcpStream;

pub struct Response<STREAM = TcpStream> {
    status: u16,
    pub headers: Headers,
    body: STREAM
}

#[allow(dead_code)]
impl<STREAM> Response<STREAM> {
    pub fn new(stream:STREAM) -> Self  {
        Self {
            status: 200,
            headers: Headers::new(),
            stream: stream
        }
    }
}