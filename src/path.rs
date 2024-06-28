use std::io::{Error, ErrorKind};
use token::token_to_regexp;
use regex::{Regex, Captures, Replacer};
use std::collections::HashMap;

mod lexer;
mod token;
mod key;

lazy_static! {
    static ref ESCAPE_REGEX:Regex = Regex::new(r"([.+*?=^!:${}()\[\]|/\\])").unwrap();
}

type Encode = fn(String)->String;
fn no_encode(s:String)->String{
    s
}
fn escape(str:String)->String {
    ESCAPE_REGEX.replace_all(&str, "\\$1");
    return str;
}

pub type Matches = HashMap<String, String>;

struct KeyMapper<'a> {
    keys: Vec<String>,
    map: &'a mut Matches
}

impl Replacer for KeyMapper<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, _dst: &mut String) {
        let mut index = 0;
        while index < self.keys.len() {
            let value = String::from(&caps[index+1]);
            self.map.insert(self.keys[index].clone(), value);
            index+=1;
        }
    }
}

impl KeyMapper<'_> {
    pub fn new<'a>(keys: &Vec<key::Key>, map: &'a mut Matches)->KeyMapper<'a>{
        let mut names: Vec<String> = Vec::new();
        for key in keys {
            names.push(key.name.clone());
        }
        KeyMapper{ keys: names, map}
    }
}


pub type PathOptions = token::CompileOptions;

pub struct Path {
    keys: Vec<key::Key>,
    regex: Regex
}

impl Path {
    pub fn new(path: String, options: PathOptions)->Result<Path, Error>{
        let data = token::parse(path, token::ParseOptions::from(&options))?;
        let mut keys = Vec::new();
        match token_to_regexp(data, &mut keys, options) {
            Ok(regex) => {
                Ok(Path{ keys, regex })
            },
            Err(e)=>Err(Error::new(ErrorKind::Other, e))
        }
    }

    pub fn match_path(&self, path:&str)->Matches {
        let mut output:HashMap<String, String> = HashMap::new();
        let map = KeyMapper::new(&self.keys, &mut output);
        self.regex.replace_all(path, map);
        return output;
    }

    pub fn keys(&self)->&Vec<key::Key> {
        &self.keys
    }
}