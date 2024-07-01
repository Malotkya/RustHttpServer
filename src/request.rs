/** Http Request
 * 
 * @author Alex Malotky
 */
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    OPTIONS,
    PATCH,
    ALL
}

impl RequestMethod {
    pub fn from(str: &str)->Option<RequestMethod> {
        if str == "GET" {
            return Some(Self::GET);
        }

        if str == "POST" {
            return Some(Self::POST);
        }

        if str == "HEAD" {
            return Some(Self::HEAD);
        }

        if str == "PUT" {
            return Some(Self::PUT);
        } 

        if str == "DELETE" {
            return Some(Self::DELETE);
        }

        if str == "OPTIONS" {
            return Some(Self::OPTIONS);
        }

        if str == "PATCH" {
            return Some(Self::PATCH);
        }

        return None;

    }

    pub fn eq(&self, value:&RequestMethod)->bool{
        if *self == Self::ALL {
            return true;
        }

        return *self == *value;
    }
}

/// Http Request
#[allow(dead_code)]
pub struct Request {
    url: String,
    method: RequestMethod,
    headers: HashMap<String, String>,
    path: String,
    pub query: String,
    search: HashMap<String, String>,
    pub body: BufReader<TcpStream>,
    params: HashMap<String, String>
}

#[allow(dead_code)]
impl Request {
    pub fn new(stream:TcpStream)->Result<Request, Error>{
        let mut buffer = BufReader::new(stream);
        let mut line = String::new();
        let mut size = buffer.read_line(&mut line).unwrap();
        let mut raw_headers: Vec<String> = Vec::new();

        //Read in Headers
        while size > 0{
            if line != "\r\n" {
                raw_headers.push(line.trim().to_string());
                line = String::new();
                size = buffer.read_line(&mut line).unwrap();
            } else {
                size = 0;
            }
        }

        if raw_headers.len() <= 0 {
            return Err(Error::new(ErrorKind::InvalidData, "No Http Headers found in Request!"));
        }

        let status:Vec<_> = (&raw_headers[0]).split(" ").collect();

        if status.len() < 2{
            return Err(Error::new(ErrorKind::InvalidData, "Malformed Http Request!"));
        }else if status[2].trim() != "HTTP/1.1" {
            return Err(Error::new(ErrorKind::InvalidData, "Request is not an Http Request!"));
        }

        //Data from incoming headers.
        let url:String = String::from(status[1]);
        let mut headers:HashMap<String, String> = HashMap::new();
        let method:RequestMethod;

        match RequestMethod::from(status[0]) {
            Some(value)=> {
                method = value;
            }
            None => {
                return Err(Error::new(ErrorKind::InvalidData, "Unknown Request Method!"));
            }
        }

        for index in 1..raw_headers.len() {
            let line:Vec<_> = raw_headers[index].split(":").collect();
            headers.insert(line[0].trim().to_string(), line[1].trim().to_string());
        }

        //Data from url
        let url_array:Vec<_> = url.split("?").collect();
        let path = String::from(url_array[0]);
        let mut search_string = String::from("");
        if url_array.len() > 1 {
            search_string = String::from(url_array[1]);
        }
        let mut search = HashMap::new();

        for string in search_string.split("&"){
            let buffer: Vec<_> = string.split("=").collect();
            if buffer.len() > 1 {
                search.insert(String::from(buffer[0].trim()), String::from(buffer[1].trim()));
            }
        }

        Ok( Self {
            method, url, headers, query: path.clone(), path, search, body: buffer, params: HashMap::new()
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }
 
    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn header(&self, key:&str) -> &str {
        &(&self.headers)[key]
    }

    pub fn set_param(&mut self, value:HashMap<String, String>){
        self.params = value;
    }

    pub fn param(&self) -> &HashMap<String, String> {
        &self.params
    }
}