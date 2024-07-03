#[macro_use]
extern crate lazy_static;
use std::io::Result;

pub mod request;
pub mod response;
pub mod http_status;
pub mod path;
pub mod router;
pub mod server;

static mut SERVER:server::Server = server::Server::new(5000);
const DATA: &str= "<form method='POST'><input name='textbox'/><br/><input type='radio' name='button' value='hello world' /><br/><button>Submit</button></form>";


fn main(){
    let mut rt = router::Router::new(path::PathOptions::default()).unwrap();
    rt.add_method(request::RequestMethod::ALL, |_req:&mut request::Request, res:&mut response::Response|->Result<()>{
        res.status(200)?;
        res.set_header(String::from("Content-Type"), String::from("text/html"))?;
        res.write(DATA.as_bytes())?;
        Ok(())
    });

    unsafe { 
        SERVER.add(Box::new(rt));

        SERVER.start(&||{
            println!("Ready on port 5000!");
        }).unwrap();
    
    };
}
