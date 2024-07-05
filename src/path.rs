/** /path
 * 
 * Based on path-to-regex:
 * https://github.com/pillarjs/path-to-regexp
 * 
 * @author Alex Malotky
 */
use std::io::{Error, ErrorKind};
use segment::compile;
use regex::{Regex, Captures, Replacer};
use std::collections::HashMap;

mod lexer;
mod token;
mod key;
mod segment;

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
pub struct MatchResult {
    pub path: String,
    pub matches: Matches
}

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


pub type PathOptions = segment::CompileOptions;

pub struct Path {
    keys: Vec<key::Key>,
    regex: Regex,
    options: PathOptions,
    value: String
}

impl Path {
    pub fn new(path: &str, options: PathOptions)->Result<Path, Error>{
        let data = token::parse(path, token::ParseOptions::from(&options))?;
        let mut keys = Vec::new();
        match compile(data, &mut keys, &options) {
            Ok(regex) => {
                Ok(Path{ keys, regex, options, value: path.to_string() })
            },
            Err(e)=>Err(Error::new(ErrorKind::Other, e))
        }
    }

    //TODO: There may be a way to make this more efficient use only one match/iterator
    pub fn match_path(&self, path:&str)->Option<MatchResult> {
        let captures = self.regex.find(path);

        if captures.is_none() {
            return None;
        }
        let url = captures.unwrap();

        let mut output:HashMap<String, String> = HashMap::new();
        let map = KeyMapper::new(&self.keys, &mut output);
        self.regex.replace_all(path, map);

        return Some(MatchResult{
            path: url.as_str().to_string(),
            matches: output
        });
    }

    pub fn keys(&self)->&Vec<key::Key> {
        &self.keys
    }

    pub fn update(&mut self, path: &String)->Result<(), Error>{
        let data = token::parse(path, token::ParseOptions::from(&self.options))?;
        self.keys = Vec::new();
        match compile(data, &mut self.keys, &self.options) {
            Err(e)=>Err(Error::new(ErrorKind::Other, e)),
            Ok(regex)=>{
                self.regex = regex;
                self.value = path.clone();
                Ok(())
            }
        }
    }

    pub fn regex_str(&self)->&str {
        self.regex.as_str()
    }

    pub fn as_str(&self)->&str {
        self.value.as_str()
    }
}