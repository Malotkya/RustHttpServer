use regex::{Regex, Error as RegexError, Captures, Replacer};
use crate::path::lexer::{ lexer, TokenType};
use crate::path::key::{Key, key_to_regexp_gen};
use crate::path::{Encode, no_encode, escape};
use std::io::Error;

pub type Token = Result<Key, String>;
fn token_string(string: String)->Token {
    Err(string)
}

fn token_key(encode: Encode, delimiter:String, name:String, pattern:Option<String>, input_prefix:Option<String>, input_suffix:Option<String>, modifier:Option<String>)->Token {
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

    Ok(
        Key{ name, prefix, suffix, separator, modifier, pattern}
    )
}

pub struct ParseOptions {
    delimiter: String,
    prefixes: String,
    encode_path: Encode
}

lazy_static! {
    static ref DEFAULT_PREFIX:String = String::from("./");
    static ref DEFAULT_DELIMITER:String = String::from("/");
}

impl ParseOptions {
    pub fn default()->ParseOptions{
        ParseOptions{
            prefixes: DEFAULT_PREFIX.to_owned(),
            delimiter: DEFAULT_DELIMITER.to_owned(),
            encode_path: no_encode
        }
    }

    pub fn from(data:&CompileOptions)->ParseOptions {
        ParseOptions{
            delimiter: data.delimiter.clone(),
            prefixes: data.prefixes.clone(),
            encode_path: data.encode_path.clone()
        }
    }
}

pub struct CompileOptions {
    pub delimiter: String,
    pub prefixes: String,
    pub encode_path: Encode,
    pub sensitive: bool,
    pub loose: String,
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
            loose:  DEFAULT_DELIMITER.to_owned(),
            end: true,
            start: true,
            trailing: true,
            sensitive: false
        }
    }

    pub fn new(prefixes:Option<String>, delimiter:Option<String>, encode_path: Option<Encode>, loose:Option<String>, start: Option<bool>, end: Option<bool>, trailing: Option<bool>, sensitive: Option<bool>)->CompileOptions {
        CompileOptions {
            prefixes: prefixes.unwrap_or(DEFAULT_PREFIX.to_owned()),
            delimiter: delimiter.unwrap_or(DEFAULT_DELIMITER.to_owned()),
            encode_path: encode_path.unwrap_or(no_encode),
            loose: loose.unwrap_or(DEFAULT_DELIMITER.to_owned()),
            start: start.unwrap_or(true),
            end: end.unwrap_or(true),
            trailing: trailing.unwrap_or(true),
            sensitive: sensitive.unwrap_or(false)
        }
    }
}

struct TokenData {
    tokens: Vec<Token>, 
    delimiter: String
}

pub fn parse(str:String, opts: ParseOptions) -> Result<TokenData, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut key = 0;
    let mut path:String = String::new();

    let mut it = lexer(str)?;
    loop {
        let char = it.try_consume(TokenType::Char);
        let name = it.try_consume(TokenType::Name);
        let pattern = it.try_consume(TokenType::Pattern);

        if char.is_some() || pattern.is_some() {
            let mut prefix = if char.is_some() {
                char.unwrap()
            } else {
                String::new()
            };
            let modifier = it.modifier();

            if !opts.prefixes.contains(&prefix){
                path += prefix.as_str();
                prefix = String::new();
            }

            if !path.is_empty() {
                tokens.push(
                    token_string((opts.encode_path)(path))
                );
                path = String::new();
            }

            let name = if name.is_none() {
                let value = key.to_string();
                key += 1;
                value
            } else {
                name.unwrap()
            };

            tokens.push(
                token_key(
                    opts.encode_path,
                    opts.delimiter.clone(),
                    name,
                    pattern,
                    Some(prefix),
                    Some(String::new()),
                    modifier
                )
            );
            continue;
        }

        let value = if char.is_some() {
            Some(char.unwrap())
        } else {
            it.try_consume(TokenType::Escaped)
        };

        if value.is_some() {
            path += &value.unwrap();
            continue;
        }

        if !path.is_empty() {
            tokens.push(
                token_string((opts.encode_path)(path))
            );
            path = String::new();
        }

        let asterisk = it.try_consume(TokenType::Asterisk);
        if asterisk.is_some() {
            let name = key.to_string();
            key += 1;

            tokens.push(
                token_key(
                    opts.encode_path,
                    opts.delimiter.clone(),
                    name,
                    Some(format!("^{}]*`", opts.delimiter.clone())), //TODO: EscpaeURI/ EncodeURI(opts.delimiter)
                    Some("".to_string()),
                    Some("".to_string()),
                    asterisk
                )
            )
        }

        let open = it.try_consume(TokenType::OpenBracket);
        if open.is_some() {
            let prefix = it.text();
            let name = it.try_consume(TokenType::Name);
            let pattern = it.try_consume(TokenType::Pattern);
            let suffix = it.text();

            it.consume(TokenType::ClosedBracket);

            let name = if name.is_some() {
                name.unwrap()
            } else {
                if pattern.is_some() {
                    let value = key.to_string();
                    key += 1;
                    value
                } else {
                    String::new()
                }
            };

            tokens.push(
                token_key(
                    opts.encode_path,
                    opts.delimiter.clone(),
                    name,
                    pattern,
                    Some(prefix),
                    Some(suffix),
                    it.modifier()
                )
            );
            continue;
        }

        it.consume(TokenType::End);
        break;
    }

    return Ok(TokenData{tokens, delimiter: opts.delimiter})
}

struct LooseReplacer;

impl Replacer for LooseReplacer {
    fn replace_append(&mut self, cap: &Captures<'_>, output:&mut String){
        let value = String::from(&cap["value"]);
        let loose = String::from(&cap["loose"]);

        if loose.is_empty() {
            output.push_str(&value);
        } else {
            output.push_str(&format!("{}+", escape(value)))
        }

    }
}

fn to_stringify(loose:String)->Result<Box<dyn Fn(String)->String>, RegexError>{
    match Regex::new(&format!("(?<value>[^{}]+|(?<loose>.))", escape(loose))) {
        Err(e)=>Err(e), 
        Ok(regex) => {
            Ok(Box::new(move |value:String|->String{
                let mut value = value.clone();
                regex.replace(&mut value, LooseReplacer);
                return value;
            }))
        }
    }
}

pub fn token_to_regexp(data:TokenData, keys:&mut Vec<Key>, options:CompileOptions)->Result<Regex, RegexError>{
    let stringify:Box<dyn Fn(String)->String> = if options.loose.is_empty() {
        Box::new(|value:String|->String{escape(value)})
    } else {
        let result = to_stringify(options.loose);
        if result.is_err() {
            let err = result.err().unwrap();
            return Err(err);
        }
        result.unwrap()
    };

    let mut key_to_regexp = key_to_regexp_gen(&stringify, data.delimiter.clone());
    let mut pattern = String::new();


    if options.start {
        pattern += "^";
    }

    for token in data.tokens {
        match token {
            Err(str)=>pattern += str.as_str(),
            Ok(token)=>{
                if !token.name.is_empty() {
                    keys.push(token.clone());
                }
                let regex = key_to_regexp(token);
                pattern += &regex;
            }
        }
    }

    if options.trailing {
        pattern += &format!("(?:{})?", stringify(data.delimiter.clone()));
    }

    if options.end {
        pattern += "$";
    } else {
        pattern += &format!("(?={}|$)", escape(data.delimiter.clone()));
    }

    return Regex::new(&pattern);
}