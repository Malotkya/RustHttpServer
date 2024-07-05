#[macro_use]
extern crate lazy_static;
use crate::router::status::{Status, next};

pub mod request;
pub mod response;
pub mod path;
pub mod router;
pub mod server;
pub mod error;

const DATA: &str= "<form method='POST'><input name='textbox'/><br/><input type='radio' name='button' value='hello world' /><br/><button>Submit</button></form>";

fn main(){

    let mut s = server::Server::new(5000);

    let mut rt = router::Router::new("/", path::PathOptions::default()).unwrap();
    rt.add_method(request::RequestMethod::ALL, |_req:&mut request::Request, res:&mut response::Response|->Status{
        res.html(DATA)?;
        next()
    });

    s.add(Box::new(rt));

    s.start(&||{
        println!("Ready on port 5000!");
    }).unwrap();
}
