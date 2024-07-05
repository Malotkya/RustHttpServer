use crate::router::layer::DynamicLayer;
use crate::request::{Request, RequestMethod};
use crate::response::Response;
use crate::error::{HttpError, HttpErrorKind};
use std::net::{TcpListener, TcpStream};
use std::io::Result;


pub struct Server {
    layers: Vec<DynamicLayer>,
    port: u32,
    
}

impl Server {
    pub const fn new(port:u32)->Server{
        Self{port, layers: Vec::new()}
    }

    pub fn add(&mut self, layer:DynamicLayer){
        self.layers.push(layer);
    }

    pub fn start(&self, callback:&dyn Fn())->Result<()>{
        let listner = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        callback();

        for connection in listner.incoming() {
            let stream = connection?;
            self.handle(stream);
        }

        Ok(())
    }

    fn handle(&self, stream:TcpStream) {
        let clone = stream.try_clone().unwrap();

        let mut request = Request::new(stream).unwrap();
        let mut response = Response::new(clone);
        let mut error: Option<HttpError> = None;

        if request.method() == RequestMethod::POST {
            let body = String::from_utf8(request.body().to_vec()).unwrap();
            println!("{}", body);
        }

        let query = String::from(request.query());
        for layer in &self.layers {
            if layer._match(&mut request) {
                match layer.handle(&mut request, &mut response) {  
                    Err(e) => {
                        error = Some(HttpError::new(HttpErrorKind::InternalServerError, e));
                        break;
                    }
                    Ok(_) => request.set_query(query.clone())
                }
            }
        }

        //404 and other error handling
        if !response.headers_sent() {
            error = Some(HttpError::new(HttpErrorKind::NotFound, String::new()));
        }

        if error.is_some() {
            let error = error.unwrap();
            println!("{}", error);
            let _ = response.status(error.code());
            response.write(error.message().as_bytes()).unwrap();
        }
    }
}