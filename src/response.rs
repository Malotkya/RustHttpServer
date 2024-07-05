/** Http Response
 * 
 * @author Alex Malotky
 */
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{Write, Error, ErrorKind};
use crate::error::HttpErrorKind;

/// Http Response
#[allow(dead_code)]
pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    headers_sent:bool,
    stream: TcpStream
}

#[allow(dead_code)]
impl Response {
    pub fn new(stream: TcpStream)->Response{
        let status = 200;
        let headers = HashMap::new();
        let headers_sent = false;

        Self{
            status, headers, headers_sent, stream
        }
    }

    fn send_headers(&mut self) {
        self.headers_sent = true;

        let mut buffer:String = String::new();
        for (key, value) in &self.headers {
            buffer += format!("{}: {}\r\n", key, value).as_str();
        }
        
        self.stream.write(format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n",
            &self.status,
            HttpErrorKind::from(self.status).as_str(), 
            buffer
        ).as_bytes()).unwrap();
    }

    pub fn status(&mut self, code: u16)->Result<(), Error>{
        if self.headers_sent {
            return Err(Error::new(ErrorKind::PermissionDenied, "Headers already sent!"))
        }

        self.status = code;
        return Ok(());
    }

    pub fn write(&mut self, data: &[u8])->Result<usize, Error>{
        if !self.headers_sent {
            self.send_headers();
        }

        return self.stream.write(data);
    }

    pub fn set_header(&mut self, key: &str, value: &str)->Result<(), Error>{
        if self.headers_sent {
            return Err(Error::new(ErrorKind::PermissionDenied, "Headers already sent!"))
        }

        self.headers.insert(key.to_string(), value.to_string());
        return Ok(());
    }

    pub fn get_header(&self, key:&str)->&str{
        &(&self.headers)[key]
    }

    pub fn headers_sent(&self)->bool {
        self.headers_sent
    }


}