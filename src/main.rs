#[macro_use]
extern crate lazy_static;


const DATA: &str= "<form method='POST'><input name='textbox'/><br/><input type='radio' name='button' value='hello world' /><br/><button>Submit</button></form>";

pub mod request;
pub mod response;
pub mod http_status;
pub mod path;
pub mod router;
pub mod server;

fn main(){
    let server = server::Server::new(5000);

    let mut rt = router::Router::new(path::PathOptions::default()).unwrap();
    rt.add_method(request::RequestMethod::ALL, |_req:&mut request::Request, res:&mut response::Response|{
        res.status(200);
        res.set_header(String::from("Content-Type"), String::from("text/plain"));
        res.write(DATA.as_bytes());
    });
    
    server.start(&||{
        println!("Ready on port 5000!");
    });

}
