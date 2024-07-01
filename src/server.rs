use crate::router::layer::DynamicLayer;
use crate::request::Request;
use crate::response::Response;
use std::net::{TcpListener, TcpStream};
use std::io::Result;
use std::thread;

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

    pub fn start(&'static self, callback:&dyn Fn())->Result<()>{
        let listner = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        callback();

        for connection in listner.incoming() {
            let stream = connection?;
            let layers = &self.layers;

            thread::spawn(||{
                Server::handle(layers, stream);
            });
        }

        Ok(())
    }

    fn handle(layers: &Vec<DynamicLayer>, stream:TcpStream) {
        let clone = stream.try_clone().unwrap();

        let mut request = Request::new(stream).unwrap();
        let mut response = Response::new(clone);

        let query = String::from(request.query());
        for layer in layers {
            if layer._match(&mut request) {
                layer.handle(&mut request, &mut response);
                request.set_query(query.clone());
            }
        }

        //404 and other error handling
    }
}