/** /router/layer
 * 
 * Represnts a list of layers
 * 
 * @author Alex Malotky
 */
use crate::path::{Path, PathOptions};
use crate::request::Request;
use crate::response::Response;
use crate::router::layer::{Layer, SingleLayer, Handler, DynamicLayer};
use crate::router::status::{next, error, Status};
use std::io::Result;

pub struct Route {
    list: Vec<Box<dyn Layer + Sync>>,
    path: Path
}

impl Route {
    pub fn new(path:&str, opts: PathOptions)->Result<Route> {
        let path = Path::new(path, opts)?;

        Ok(Self{ path, list: Vec::new()})
    }

    pub fn dyn_new(path:&str, opts: PathOptions)->Result<DynamicLayer> {
        let route = Self::new(path, opts)?;
        Ok(Box::new(route))
    }

    pub fn default_options()->PathOptions {
        PathOptions::new(
            None,
            None,
            None,
            None,
            Some(false),
            None,
            None
        )
    }

    pub fn add_handler(&mut self, path:&str, handler:Handler)->Result<()>{
        self.add_layer(SingleLayer::dyn_new(path, PathOptions::default(), handler)?);
        Ok(())
    }

    pub fn add_layer(&mut self, layer:Box<dyn Layer + Sync>){
        self.list.push(layer);
    } 
}

impl Layer for Route {
    fn _match(&self, req:&mut Request)->bool {
        match self.path.match_path(req.query()) {
            None => false,
            Some(result) => {
                req.set_param(result.matches);
                req.set_query(req.query().replace(&result.path, ""));
                return true;
            }
        }
    }

    fn handle(&self, req: &mut Request, res: &mut Response)->Status{
        let query = String::from(req.query());
        for layer in &self.list {
            if layer._match(req) {
                let err = layer.handle(req, res)?;
                if err.is_some() {
                    return error(err.unwrap())
                }
                req.set_query(query.clone());
            }
        }
        req.set_query(query);
        next()
    }

    fn path(&self) ->&str {
        self.path.as_str()
    }

    fn set_path(&mut self, value: &String)->Result<()>{
        self.path.update(value)
    }
}