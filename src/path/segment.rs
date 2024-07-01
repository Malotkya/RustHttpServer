/** /path/segment
 * 
 * @author Alex Malotky
 */
use crate::path::{Encode, no_encode, escape};
use crate::path::token::{DEFAULT_PREFIX, DEFAULT_DELIMITER};
use crate::path::key::{Key, key_to_regexp_gen};
use regex::{Regex, Error as RegexError};

pub struct CompileOptions {
    pub delimiter: String,
    pub prefixes: String,
    pub encode_path: Encode,
    pub sensitive: bool,
    pub end: bool,
    pub start: bool,
    pub trailing: bool
}

impl CompileOptions {
    pub fn default()->CompileOptions{
        CompileOptions{
            prefixes: DEFAULT_PREFIX.to_owned(),
            delimiter: DEFAULT_DELIMITER.to_owned(),
            encode_path: no_encode,
            end: true,
            start: true,
            trailing: true,
            sensitive: false
        }
    }

    pub fn new(prefixes:Option<String>, delimiter:Option<String>, encode_path: Option<Encode>, start: Option<bool>, end: Option<bool>, trailing: Option<bool>, sensitive: Option<bool>)->CompileOptions {
        CompileOptions {
            prefixes: prefixes.unwrap_or(DEFAULT_PREFIX.to_owned()),
            delimiter: delimiter.unwrap_or(DEFAULT_DELIMITER.to_owned()),
            encode_path: encode_path.unwrap_or(no_encode),
            start: start.unwrap_or(true),
            end: end.unwrap_or(true),
            trailing: trailing.unwrap_or(true),
            sensitive: sensitive.unwrap_or(false)
        }
    }
}

pub enum Segment {
    Key(Key),
    Str(String)
}

impl Segment {
    pub fn new(encode: Encode, delimiter:String, name:String, pattern:Option<String>, input_prefix:Option<String>, input_suffix:Option<String>, modifier:Option<String>)->Segment {
        let prefix = encode(input_prefix.unwrap_or(String::new()));
        let suffix = encode(input_suffix.unwrap_or(String::new()));
        let modifier = modifier.unwrap_or(String::new());
        let pattern = pattern.unwrap_or(String::new());
    
        let separator:Option<String> = if modifier == "*" || modifier == "+" {
            let postfix:String = if !suffix.is_empty()  {
                String::from(&suffix)
            } else {
                delimiter
            };
            Some(format!("{}{}", prefix, postfix))
        } else {
            None
        };
    
        Segment::Key(
            Key{ name, prefix, suffix, separator, modifier, pattern}
        )
    }
}


pub struct RegexData {
    pub tokens: Vec<Segment>, 
    pub delimiter: String
}

pub fn compile(data:RegexData, keys:&mut Vec<Key>, options:&CompileOptions)->Result<Regex, RegexError>{
    let mut key_to_regexp = key_to_regexp_gen(data.delimiter.clone());
    let mut pattern = String::new();


    if options.start {
        pattern += "^";
    }

    for token in data.tokens {
        match token {
            Segment::Str(str)=>pattern += str.as_str(),
            Segment::Key(token)=>{
                if !token.name.is_empty() {
                    keys.push(token.clone());
                }
                let regex = key_to_regexp(token);
                pattern += &regex;
            }
        }
    }

    if options.trailing {
        pattern += &format!("(?:{})?", escape(data.delimiter.clone()));
    }

    if options.end {
        pattern += "$";
    } else {
        pattern += &format!("(?={}|$)", escape(data.delimiter.clone()));
    }

    return Regex::new(&pattern);
}