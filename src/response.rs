/** Http Response
 * 
 * @author Alex Malotky
 */
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{Write, Error, ErrorKind};
use crate::http_status;

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
            http_status::get_message(self.status), 
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

    pub fn set_header(&mut self, key: String, value: String)->Result<(), Error>{
        if self.headers_sent {
            return Err(Error::new(ErrorKind::PermissionDenied, "Headers already sent!"))
        }

        self.headers.insert(key, value);
        return Ok(());
    }

    pub fn get_header(&self, key: String)->Option<&String>{
        return self.headers.get(&key);
    }


}