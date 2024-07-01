/** /path/token
 * 
 * @author Alex Malotky
 */
use crate::path::lexer::lexer;
use crate::path::{Encode, escape};
use crate::path::segment::{Segment, CompileOptions, RegexData};
use std::io::{Error, ErrorKind};

pub struct ParseOptions {
    delimiter: String,
    encode_path: Encode
}

lazy_static! {
    pub static ref DEFAULT_PREFIX:String = String::from("./");
    pub static ref DEFAULT_DELIMITER:String = String::from("/");
}

impl ParseOptions {
    pub fn from(data:&CompileOptions)->ParseOptions {
        ParseOptions{
            delimiter: data.delimiter.clone(),
            encode_path: data.encode_path.clone()
        }
    }
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
pub struct Token {
    pub token_type: TokenType,
    _index: usize,
    pub value: String,
}

impl Token{
    pub fn new(token_type: TokenType, index:usize, value:String)->Token {
        Token{token_type, value, _index: index}
    }
}

pub fn parse(str:&String, opts: ParseOptions) -> Result<RegexData, Error> {
    let mut tokens: Vec<Segment> = Vec::new();
    let mut key = 0;

    let mut it = lexer(str)?;
    loop {
        let path = it.text();
        if !path.is_empty() {
            tokens.push(Segment::Str(escape(path)));
        }

        let name = it.try_consume(TokenType::Name);
        let pattern = it.try_consume(TokenType::Pattern);

        if name.is_some() || pattern.is_some() {
            let string = name.unwrap_or(key.to_string());
            key += 1;

            tokens.push(
                Segment::new(
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
                Segment::new(
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
                Segment::new(
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

    return Ok(RegexData{tokens, delimiter: opts.delimiter})
}