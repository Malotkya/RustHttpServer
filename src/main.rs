use std::net::TcpListener;

mod request;
mod response;
mod http_status;

fn server(listner: TcpListener){
    for stream in listner.incoming() {
        let stream = stream.unwrap();

        let _request = request::Request::new(&stream).unwrap();
        let mut response = response::Response::new(stream);
        response.status(404).unwrap();
        response.write("Hello World".as_bytes()).unwrap();
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
