use std::net::TcpListener;

mod request;

fn server(listner: TcpListener){
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                let request = request::Request::new(stream).unwrap();

                println!("{}", request.path());
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
