use regex::{Regex, Error as RegexError};
use crate::path::lexer::{ lexer, TokenType};
use crate::path::key::{Key, key_to_regexp_gen};
use crate::path::{Encode, no_encode, escape};
use std::io::{Error, ErrorKind};

pub enum Token {
    Key(Key),
    Str(String)
}

impl Token {
    pub fn new(encode: Encode, delimiter:String, name:String, pattern:Option<String>, input_prefix:Option<String>, input_suffix:Option<String>, modifier:Option<String>)->Token {
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
    
        Token::Key(
            Key{ name, prefix, suffix, separator, modifier, pattern}
        )
    }
}

pub struct ParseOptions {
    delimiter: String,
    encode_path: Encode
}

lazy_static! {
    static ref DEFAULT_PREFIX:String = String::from("./");
    static ref DEFAULT_DELIMITER:String = String::from("/");
}

impl ParseOptions {
    pub fn from(data:&CompileOptions)->ParseOptions {
        ParseOptions{
            delimiter: data.delimiter.clone(),
            encode_path: data.encode_path.clone()
        }
    }
}

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

pub struct TokenData {
    tokens: Vec<Token>, 
    delimiter: String
}

pub fn parse(str:String, opts: ParseOptions) -> Result<TokenData, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut key = 0;

    let mut it = lexer(str)?;
    loop {
        let path = it.text();
        if !path.is_empty() {
            tokens.push(Token::Str(escape(path)));
        }

        let name = it.try_consume(TokenType::Name);
        let pattern = it.try_consume(TokenType::Pattern);

        if name.is_some() || pattern.is_some() {
            let string = name.unwrap_or(key.to_string());
            key += 1;

            tokens.push(
                Token::new(
                    opts.encode_path,
                    opts.delimiter.clone(),
                    string,
                    pattern,
                    None,
                    None,
                    None
                )
            );

            continue;
        }

        let asterisk = it.try_consume(TokenType::Asterisk);
        if asterisk.is_some() {
            let name = key.to_string();
            key += 1;

            tokens.push(
                Token::new(
                    opts.encode_path,
                    opts.delimiter.clone(),
                    name,
                    Some(format!("^{}]*", opts.delimiter.clone())), //TODO: EscpaeURI/ EncodeURI(opts.delimiter)
                    Some(String::new()),
                    Some(String::new()),
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

            it.consume(TokenType::ClosedBracket).unwrap();

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
                Token::new(
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

        if it.consume(TokenType::End).is_err() {
            let next = it.peek();
            return Err(Error::new(ErrorKind::UnexpectedEof, format!("Expected End instead got {}!", next.token_type.as_str())));
        }
        break;
    }

    return Ok(TokenData{tokens, delimiter: opts.delimiter})
}

pub fn token_to_regexp(data:TokenData, keys:&mut Vec<Key>, options:CompileOptions)->Result<Regex, RegexError>{
    let mut key_to_regexp = key_to_regexp_gen(data.delimiter.clone());
    let mut pattern = String::new();


    if options.start {
        pattern += "^";
    }

    for token in data.tokens {
        match token {
            Token::Str(str)=>pattern += str.as_str(),
            Token::Key(token)=>{
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