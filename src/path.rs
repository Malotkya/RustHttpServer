use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use regex::Regex;

mod lexer;
mod token;
mod key;

lazy_static! {
    static ref ESCAPE_REGEX:Regex = Regex::new(r#"/([.+*?=^!:${}()[\]|/\\])"#).unwrap();
}

type Encode = fn(String)->String;
fn no_encode(s:String)->String{
    s
}
fn escape(str:String)->String {
    ESCAPE_REGEX.replace_all(&str, "\\$1");
    return str;
}

pub type PathOptions = token::CompileOptions;

pub struct Path {
    keys: Vec<key::Key>,
    regex: Regex
}

impl Path {
    pub fn new(path: String, options: PathOptions)->Result<Path, Error>{
        let data = token::parse(path, token::ParseOptions::from(options))?;
        let mut keys = Vec::new();
        match token::token_to_regexp(data, &mut keys, options) {
            Ok(regex) => Ok(Path{ keys, regex }),
            Err(e)=>Err(Error::new(ErrorKind::Other, e))
        }
    }

    pub fn match(&self, path:&String)->Vec<Option<String>> {

    }

    pub fn keys(&self)->&Vec<key::Key> {
        &self.keys
    }
}