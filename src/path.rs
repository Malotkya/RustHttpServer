use std::collections::HashMap;
use std::io::Result;

mod lexer;
mod token;
mod key;

pub type ParamData = Option<HashMap<String, Vec<String>>>;
pub type PathFunction = Box<dyn FnMut(ParamData)->Result<String>>;