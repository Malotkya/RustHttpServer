use regex::Regex;

use crate::path::lexer::{ lexer, TokenType};
use crate::path::key::Key;
use crate::path::PathFunction;
use std::io::{Error, ErrorKind};

type Encode = fn(String)->String;
fn no_encode(s:String)->String{
    s
}

pub type Token = Result<Key, String>;
fn token_string(string: String)->Token {
    Err(string)
}
fn token_key(encode: Encode, delimiter:String, name:String, pattern:Option<String>, input_prefix:Option<String>, input_suffix:Option<String>, modifier:Option<String>)->Token {
    let prefix = encode(input_prefix.unwrap_or("".to_string()));
    let suffix = encode(input_suffix.unwrap_or("".to_string()));
    let modifier = modifier.unwrap_or("".to_string());
    let pattern = pattern.unwrap_or("".to_string());

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

impl ParseOptions {
    pub fn Default()->ParseOptions{
        ParseOptions{
            prefixes: String::from("./"),
            delimiter: String::from("/"),
            encode_path: no_encode
        }
    }
}

pub struct CompileOptions {
    delimiter: String,
    prefixes: String,
    encode_path: Encode,
    sensitive: bool,
    loose: String,
    validate:bool,
    encode: Option<Encode>
}

impl CompileOptions {
    pub fn Default()->CompileOptions{
        CompileOptions{
            prefixes: String::from("./"),
            delimiter: String::from("/"),
            encode_path: no_encode,
            encode: None, //TODO: encodeURIComponent,
            loose:  String::from("/"),
            validate: true,
            sensitive: false
        }
    }
}

struct TokenData {
    tokens: Vec<Token>, 
    delimter: String
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
            let patter = it.try_consume(TokenType::Pattern);
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

    return Ok(TokenData{tokens, delimter: opts.delimiter})
}

pub fn token_to_function(token:Token, encode: Option<Encode>)->PathFunction {
    match token {
        Err(string)=> Box::new(move |_ignore|{return Ok(string.clone());}),
        Ok(token)=>{

            let optional:bool = token.modifier == "?" || token.modifier == "*";
            let encode_value: Encode = if encode.is_some() {
                encode.unwrap()
            } else {
                no_encode
            };

            if encode.is_some() && token.separator.is_some() {
                //let stringify = encode_value;
                let separator = token.separator.unwrap();

                if optional {
                    return Box::new(move |data|{
                        if data.is_none() {
                            return Ok(String::new());
                        }
                        let data = data.unwrap();
                        let value = data.get(&token.name);
                        if value.is_none() {
                            return Ok(String::new());
                        }

                        let value = value.unwrap();
                        if value.len() == 0 {
                            return Ok(String::new());
                        }

                        let mut output = token.prefix.clone();
                        for string in value {
                            output += string.as_str();
                            output += separator.as_str();
                        }
                        return Ok(output + token.suffix.as_str());
                    })
                }

                return Box::new(move |data|{
                    if data.is_none(){
                        return Err(Error::new(ErrorKind::NotFound, "Data is Undefind!"));
                    }
                    let data = data.unwrap();
                    let value = data.get(&token.name);
                    if value.is_none() {
                        return Err(Error::new(ErrorKind::NotFound, format!("'{}' not found in data!", &token.name)))
                    }
                    let value = value.unwrap();
                    if value.len() == 0 {
                        return Ok(String::new());
                    }

                    let mut output = token.prefix.clone();
                    for string in value {
                        output += encode_value(string.to_string()).as_str();
                        output += separator.as_str();
                    }
                    return Ok(output + token.suffix.as_str());
                })
            }

            if optional {
                return Box::new(move |data|{
                    if data.is_none(){
                        return Err(Error::new(ErrorKind::NotFound, "Data is Undefind!"));
                    }
                    let data = data.unwrap();
                    let value = data.get(&token.name);
                    if(value.is_none()){
                        return Ok(String::new());
                    }
                    let value = value.unwrap();
                    let mut output:String = token.prefix.clone();
                    output += encode_value(value[0].to_string()).as_str();
                    output += token.suffix.as_str();
                    return Ok(output);
                });
            }

            return Box::new(move |data|{
                if data.is_none(){
                    return Err(Error::new(ErrorKind::NotFound, "Data is Undefind!"));
                }
                let data = data.unwrap();
                let value = data.get(&token.name);
                if value.is_none() {
                    return Err(Error::new(ErrorKind::NotFound, format!("'{}' not found in data!", &token.name)))
                }
                let value = value.unwrap();
                let mut output:String = token.prefix.clone();
                output += encode_value(value[0].to_string()).as_str();
                output += token.suffix.as_str();
                return Ok(output);
            });
        }
    }
}