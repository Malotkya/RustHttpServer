use std::net::TcpListener;

const DATA: &str= "<form method='POST'><input name='textbox'/><br/><input type='radio' name='button' value='hello world' /><br/><button>Submit</button></form>";

pub mod request;
pub mod response;
pub mod http_status;
//pub mod router;

fn server(listner: TcpListener){
    for stream in listner.incoming() {
        let stream = stream.unwrap();

        let _request = request::Request::new(&stream).unwrap();
        let mut response = response::Response::new(stream);
        response.status(404).unwrap();
        response.write(DATA.as_bytes()).unwrap();
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
