use std::net::{TcpListener, TcpStream};
use std::io::{ prelude::*, BufReader};

fn handle_stream(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    println!("Buffer: {buf_reader:#?}\n");

    let lines = buf_reader.lines();
    //println!("Lines: {lines:#?}]n");

    let result = lines.map(|result| result.unwrap());
    //println!("Result: {result:#?}]n");
        
    let http_request: Vec<_> = result.take_while(|line| !line.is_empty()).collect();
    //println!("Request: {http_request:#?}");

    let status_line  = "HTTP/1.1 200 OK";
    let message = "Hello World";
    
    let response = format!("{status_line}'\r\n\r\n{message}");
    stream.write_all(response.as_bytes()).unwrap();

        
}

fn server(listner: TcpListener){
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream);
            }
            Err(e) => {
                panic!("There was an issue with the incoming message:\n{}", e);
            }
        }
    }
}

fn main(){
    let listner = TcpListener::bind("127.0.0.1:5000");

    match listner {
        Ok(listner) => {
            println!("Ready on port 5000!");
            server(listner);
        }
        Err(e) => {
            panic!("There was a problem starting that application:\n{}", e);
        }
    }

    
}
