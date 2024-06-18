/** Http Request
 * 
 * @author Alex Malotky
 */
use std::io::{ prelude::*, BufReader, Error, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;

/// Http Request
#[allow(dead_code)]
pub struct Request {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    path: String,
    query: HashMap<String, String>

}

#[allow(dead_code)]
impl Request {
    pub fn new(mut stream: &TcpStream)->Result<Request, Error>{
        let buffer: Vec<_> = BufReader::new(&mut stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let status:Vec<_> = (&buffer[0]).split(" ").collect();

        if status.len() < 2{
            return Err(Error::new(ErrorKind::InvalidData, "Malformed Http Request!"));
        }else if status[2].trim() != "HTTP/1.1" {
            return Err(Error::new(ErrorKind::InvalidData, "Request is not an Http Request!"));
        }

        //Data from incoming headers.
        let method:String = String::from(status[0]);
        let url:String = String::from(status[1]);
        let mut headers:HashMap<String, String> = HashMap::new();

        for index in 1..buffer.len() {
            let line:Vec<_> = buffer[index].split(":").collect();
            println!("{}: {}", line[0], line[1]);
            headers.insert(line[0].trim().to_string(), line[1].trim().to_string());
        }

        //Data from url
        let url_array:Vec<_> = url.split("?").collect();
        let path = String::from(url_array[0]);
        let search_string = String::from(url_array[1]);
        let mut query = HashMap::new();

        for string in search_string.split("&"){
            let buffer: Vec<_> = string.split("=").collect();
            query.insert(String::from(buffer[0].trim()), String::from(buffer[1].trim()));
        }

        Ok( Self {
            method, url, headers, path, query
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn header(&self, key:&str) -> &str {
        &(&self.headers)[key]
    }
}