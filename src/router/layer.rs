/** /router/layer
 * 
 * Represnts a single layer
 * 
 * @author Alex Malotky
 */
use crate::path::{Path, PathOptions};
use crate::request::Request;
use crate::response::Response;
use std::io::Result;

pub type Handler = fn(req: &mut Request, res: &mut Response)->Result<()>;

pub trait Layer {
    fn _match(&self, req:&mut Request)->bool;
    fn path(&self)->&str;
    fn set_path(&mut self, value: &String)->Result<()>;
    fn handle(&self, req:&mut Request, res: &mut Response)->Result<()>;
}

pub struct SingleLayer {
    path: Path,
    handler: Handler,
}

pub type DynamicLayer = Box<dyn Layer + Sync>;

impl SingleLayer {
    pub fn new(path:String, options: PathOptions, handler:Handler) -> Result<SingleLayer>{
        let path = Path::new(path, options)?;

        return Ok(Self{path, handler});
    }

    pub fn dyn_new(path:String, options: PathOptions, handler:Handler)->Result<DynamicLayer> {
        let layer = Self::new(path, options, handler)?;
        Ok(Box::new(layer))
    }
}

impl Layer for SingleLayer{
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

    fn handle(&self, req:&mut Request, res: &mut Response)->Result<()>{
        (self.handler)(req, res)
    }

    fn path(&self) ->&str {
        self.path.as_str()
    }

    fn set_path(&mut self, value: &String)->Result<()>{
        self.path.update(value)
    }
}