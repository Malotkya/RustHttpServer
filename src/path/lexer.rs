/** /path/lexer
 * 
 * @author Alex Malotky
 */
use std::io::{Error, ErrorKind};
use regex::Regex;

lazy_static! {
    static ref CHAR_REGEX:Regex = Regex::new(r#"^\p{XID_Continue}$"#).unwrap();
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Name = 0,
    Pattern,
    Char,
    Escaped,
    End,
    OpenBracket,
    ClosedBracket,
    Asterisk,
    Plus,
    Exclimation,
    SemiColon,
    AtSymbol,
    QuestionMark
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Name => "NAME",
            TokenType::Pattern => "PATTERN",
            TokenType::Char => "CHAR",
            TokenType::Escaped => "ESCAPED",
            TokenType::End => "END",
            TokenType::Exclimation => "!",
            TokenType::SemiColon => ";",
            TokenType::AtSymbol => "@",
            TokenType::OpenBracket => "{",
            TokenType::ClosedBracket => "}",
            TokenType::Asterisk => "*",
            TokenType::Plus => "+",
            TokenType::QuestionMark => "?"
        }
    }

    pub fn from(c: char) -> Option<TokenType> {
        if c == '{' {
            return Some(TokenType::OpenBracket);
        }

        if c == '}' {
            return Some(TokenType::ClosedBracket);
        }

        if c == '*' {
            return Some(TokenType::Asterisk);
        }

        if c == '+' {
            return Some(TokenType::Plus);
        }

        if c == '?' {
            return Some(TokenType::QuestionMark);
        }

        if c == '!' {
            return Some(TokenType::Exclimation);
        }

        if c == '@' {
            return Some(TokenType::AtSymbol);
        }

        if c == ';' {
            return Some(TokenType::SemiColon);
        }

        return None;
            
    }

    pub fn eq(&self, compare:&TokenType)->bool {
        *self as u8 == *compare as u8
    }
}

#[derive(Debug, Clone)]
pub struct LexToken {
    pub token_type: TokenType,
    _index: usize,
    pub value: String,
}

pub fn lexer(string: String)->Result<Iter, Error>{
    let chars: Vec<char> = string.chars().collect();
    let mut tokens: Vec<LexToken> = Vec::new();
    let mut index = 0;

    while index < chars.len(){
        let value = chars[index];

        match TokenType::from(value) {
            Some(token)=>{
                tokens.push(LexToken { token_type: token, _index:index, value: value.to_string() });
                index+=1;
                continue;
            }
            None=>{}
        }

        if value == '\\' {
            tokens.push(LexToken { token_type: TokenType::Escaped, _index:index, value: value.to_string() });
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
                return Err(Error::new(ErrorKind::Other, format!("Missing parameter name at {}", index)));
            }

            tokens.push(LexToken{token_type: TokenType::Name, _index:index, value: name});
            index +=1;
            continue;
        }

        if value == '(' {
            let pos = index;
            index+=1;
            let mut count: usize = 1;
            let mut pattern:String = String::new();

            if chars[index] == '?'{
                return Err(Error::new(ErrorKind::Other, format!("Pattern cannot start with '?' at {}", index)));
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
                            return Err(Error::new(ErrorKind::Other, format!("Capturing groups are not allowed at {}", index)));
                        }
                    }
                }

                pattern += &String::from(chars[index+1]);
                index += 1;
            }

            if count != 0 {
                return Err(Error::new(ErrorKind::Other, format!("Unbalanced pattern at {}", pos)));
            }
            if pattern.is_empty() {
                return Err(Error::new(ErrorKind::Other, format!("Missing pattern at {}", pos)));
            }

            tokens.push(LexToken{token_type: TokenType::Pattern, _index:index, value: pattern});
            continue;
        }

        tokens.push(LexToken{token_type: TokenType::Char, _index:index, value: value.to_string()});
        index += 1;
        
    }

    tokens.push(LexToken{ token_type: TokenType::End, _index:index, value: String::new() });

    Ok(Iter::new(tokens))
}

pub struct Iter {
    index: usize,
    tokens: Vec<LexToken>
}

impl Iter {
    pub fn new(tokens: Vec<LexToken>)-> Iter {
        Iter { tokens, index: 0}
    }

    pub fn peek(&self)->&LexToken{
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