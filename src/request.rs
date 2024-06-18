use std::io::{ prelude::*, BufReader, Error, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;

pub struct Request {
    path: String,
    method: String,
    headers: HashMap<String, String>
}

/** Request
 * 
 * Currently only handles get requests.
 */
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
        }/* else if status[2].trim() == "HTTP/1.1" {
            println!("{:?}", status[2]);
            return Err(Error::new(ErrorKind::InvalidData, "Request is not an Http Request!"));
        }*/

        let method:String = String::from(status[0]);
        let path:String = String::from(status[1]);
        let mut headers:HashMap<String, String> = HashMap::new();

        for index in 1..buffer.len() {
            let line:Vec<_> = buffer[index].split(":").collect();
            headers.insert(line[0].trim().to_string(), line[1].trim().to_string());
        }

        Ok( Self {
            method, path, headers
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