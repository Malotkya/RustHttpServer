/** /path/lexer
 * 
 * @author Alex Malotky
 */
use crate::path::token::{Token, TokenType};
use std::io::{Error, ErrorKind};
use regex::Regex;

lazy_static! {
    static ref CHAR_REGEX:Regex = Regex::new(r#"^\p{XID_Continue}$"#).unwrap();
}

pub fn lexer(string: &String)->Result<Iter, Error>{
    let chars: Vec<char> = string.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0;

    while index < chars.len(){
        let value = chars[index];

        match TokenType::from(value) {
            Some(token)=>{
                tokens.push(Token::new(token, index, value.to_string()));
                index+=1;
                continue;
            }
            None=>{}
        }

        if value == '\\' {
            tokens.push(Token::new(TokenType::Escaped, index, value.to_string()));
            index+=1;
            continue;
        }

        if value == ':' {
            let mut name:String = String::new();

            while CHAR_REGEX.is_match(&String::from(*chars.get(index+1).unwrap_or(&' '))){
                index+=1;
                name += &String::from(chars[index]);
            }

            if name.is_empty() {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Missing parameter name at {}", index)));
            }

            tokens.push(Token::new(TokenType::Name, index, name ));
            index +=1;
            continue;
        }

        if value == '(' {
            let pos = index;
            index+=1;
            let mut count: usize = 1;
            let mut pattern:String = String::new();

            if chars[index] == '?'{
                return Err(Error::new(ErrorKind::InvalidInput, format!("Pattern cannot start with '?' at {}", index)));
            }

            while index < chars.len() {
                if chars[index] == '\\' {
                    pattern += &String::from(chars[index]);
                    pattern += &String::from(chars[index+1]);
                    index += 2;
                    continue;
                }

                if chars[index] == ')' {
                    count -= 1;
                    if count == 0 {
                        index+=1;
                        break;
                    } else if chars[index] == '(' {
                        count += 1;
                        if chars[index+1] != '?' {
                            return Err(Error::new(ErrorKind::InvalidInput, format!("Capturing groups are not allowed at {}", index)));
                        }
                    }
                }

                pattern += &String::from(chars[index+1]);
                index += 1;
            }

            if count != 0 {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Unbalanced pattern at {}", pos)));
            }
            if pattern.is_empty() {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Missing pattern at {}", pos)));
            }

            tokens.push(Token::new(TokenType::Pattern, index, pattern ));
            continue;
        }

        tokens.push(Token::new(TokenType::Char, index, value.to_string()));
        index += 1;
        
    }

    tokens.push(Token::new(TokenType::End, index, String::new()));

    Ok(Iter::new(tokens))
}

pub struct Iter {
    index: usize,
    tokens: Vec<Token>
}

impl Iter {
    pub fn new(tokens: Vec<Token>)-> Iter {
        Iter { tokens, index: 0}
    }

    pub fn peek(&self)->&Token{
        &self.tokens[self.index]
    }

    pub fn try_consume(&mut self, token_type:TokenType)->Option<String>{
        let token = &self.tokens[self.index];
        if !token.token_type.eq(&token_type) {
            return None;
        }
        self.index += 1;
        return Some(token.value.clone());
    }

    pub fn consume(&mut self, token_type:TokenType)->Result<String, Error>{
        let value = self.try_consume(token_type);
        match value {
            Some(value)=> {return Ok(value)}
            None=>{
                let next = self.peek();
                return Err(Error::new(ErrorKind::InvalidInput, format!(
                    "Unexpected {} at {}, expected {}", next.token_type.as_str(), self.index, token_type.as_str()
                )))
            }
        }
    }

    pub fn modifier(&mut self)->Option<String> {
        match self.try_consume(TokenType::QuestionMark) {
            Some(value) => {return Some(value)}
            None => {
                match self.try_consume(TokenType::Asterisk) {
                    Some(value)=>{return Some(value)}
                    None => {
                        return self.try_consume(TokenType::Plus);
                    }
                }

            }
        }
    }

    fn next(&mut self)->Option<String>{
        match self.try_consume(TokenType::Char) {
            Some(value) => Some(value),
            None => {
                return self.try_consume(TokenType::Escaped);
            }
        }
    }

    pub fn text(&mut self)->String {
        let mut result = String::new();
        let mut value: Option<String> = self.next();
        while value.is_some(){
            result += &value.unwrap();
            value = self.next();
        }
        return result;
    }
}