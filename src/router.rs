/** /router
 * 
 * Represnts a layer end point
 * 
 * @author Alex Malotky
 */
use crate::path::{Path, PathOptions};
use crate::request::{Request, RequestMethod};
use crate::router::layer::{Layer, SingleLayer, Handler, DynamicLayer};
use crate::response::Response;
use std::io::Result;

pub mod layer;
pub mod route;

struct Method {
    pub name: RequestMethod,
    pub layer: DynamicLayer
}

pub struct Router {
    list: Vec<Method>,
    path: Path
}

impl Router {
    pub fn new(opts: PathOptions)->Result<Router> {
        let path = Path::new(String::from("/"), opts)?;

        Ok(Self{list: Vec::new(), path})
    }

    pub fn dyn_new(opts: PathOptions)->Result<DynamicLayer> {
        let router = Self::new(opts)?;
        Ok(Box::new(router))
    }

    fn add(&mut self, method: RequestMethod, layer:DynamicLayer){
        self.list.push(Method{
            name: method,
            layer
        });
    }

    pub fn add_method(&mut self, method: RequestMethod, handler:Handler){
        let layer = SingleLayer::new(String::from("/"), PathOptions::default(), handler).unwrap();
        self.add(method, Box::new(layer));
    }

    pub fn add_layer(&mut self, layer:DynamicLayer) {
        self.add(RequestMethod::ALL, layer);
    }
}

impl Layer for Router {
    fn _match(&self, req:&mut Request)->bool {
        match self.path.match_path(&req.query) {
            None => false,
            Some(result) => {
                req.set_param(result.matches);
                req.query = req.query.replace(&result.path, "");
                return true;
            }
        }
    }

    fn handle(&self, req: &mut Request, res: &mut Response){
        let query = req.query.clone();
        for item in &self.list {
            if item.name.eq(req.method()) && item.layer._match(req) {
                item.layer.handle(req, res);
                req.query = query.clone();
            }
        }
        req.query = query;
    }

    fn path(&self) ->&str {
        self.path.as_str()
    }

    fn set_path(&mut self, value: &String)->Result<()>{
        self.path.update(value)
    }
}