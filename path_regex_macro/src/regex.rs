/// https://github.com/pillarjs/path-to-regexp/blob/master/src/index.ts#L503
use regex::{Regex, RegexBuilder};
use std::collections::VecDeque;
use urlencoding::encode;

//const PREFIX:&'static str    = "./";
const DELIMITER:&'static str = "/";
const ESCAPED_DELIMITER:&'static str = "\\/";
const EMPTY_STRING:&'static str = "";

lazy_static::lazy_static! {
    static ref ID_START:Regex = RegexBuilder::new(r#"^[$_\p{ID_Start}]$"#)
            .case_insensitive(true).build().unwrap();

    static ref ID_CONTINUE:Regex = RegexBuilder::new(r#"^[$\u200c\u200d\p{ID_Continue}]$"#)
            .case_insensitive(true).build().unwrap();

    static ref ESCAPE:Regex = RegexBuilder::new(r#"[.+*?^${}()\[\]|/\\]"#)
            .build().unwrap();
}

#[derive(PartialEq, Eq)]
pub(crate) enum TokenType {
    OpenBracket,
    ClosedBracket,
    WildCard,
    Param,
    Char,
    Escaped,
    End,
    Reserved
}

impl TokenType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TokenType::OpenBracket => "{",
            TokenType::ClosedBracket => "}",
            TokenType::WildCard => "WILDCARD",
            TokenType::Param => "PARAM",
            TokenType::Char => "CHAR",
            TokenType::Escaped => "ESCAPED",
            TokenType::End => "END",
            TokenType::Reserved => "RESERVED"
        }
    }

    pub(crate) fn from(c: char) -> TokenType {
        match c {
            '{' => TokenType::OpenBracket,
            '}' => TokenType::ClosedBracket,
            '*' => TokenType::WildCard,
            ':' => TokenType::Param,
            '\\' => TokenType::Escaped,

            // Reserved for use or ambiguous due to past use.
            '(' => TokenType::Reserved,
            ')' => TokenType::Reserved,
            '[' => TokenType::Reserved,
            ']' => TokenType::Reserved,
            '+' => TokenType::Reserved,
            '?' => TokenType::Reserved,
            '!' => TokenType::Reserved,

            //Default
            _ => TokenType::Char
        }
    }
}

pub(crate) struct Token {
    pub token_type: TokenType,
    index: usize,
    pub value: String,
}

pub(crate) fn lexer<'a>(str: &'a str)->Iter {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let chars: Vec<char> = str.chars().collect();
    let length = chars.len();
    let mut index = 0;

    let name = |mut i:usize| -> (String, usize) {
        let mut value = String::new();

        i += 1;
        let mut char = chars[i].to_string();
        if ID_START.is_match(&char) {
            value += &char;
             
            i+=1;
            while let Some(next) = chars.get(i) {
                let char = next.to_string();
                
                if !ID_CONTINUE.is_match(&char) {
                    break;
                }

                value += &char;
                i+=1;
            }
        } else if char == "\"" {
            let mut pos = i;

            while i < length {
                i += 1;
                char = chars[i].to_string();

                if char == "\"" {
                    i += 1;
                    pos = 0;
                    break;
                }

                if char == "\\" {
                    i += 1;
                    value += &chars[i].to_string();
                } else {
                    value += &char.to_string()
                }
            } //End While

            if pos > 0 {
                panic!(
                    "Unterminated quote at {}!",
                    pos
                )
            }
        }

        if value.is_empty() {
            panic!(
                "Missing parameter name at {}!",
                i
            )
        }

        (value, i)
    };

    while index < length {
        let value = chars[index];

        let token = TokenType::from(value);
        match token {
            TokenType::ClosedBracket |
            TokenType::OpenBracket => {
                tokens.push_back(Token {
                    token_type: token,
                    index,
                    value: value.to_string()
                });
                index += 1;
            },
            TokenType::Escaped => {
                tokens.push_back(Token {
                    token_type: token,
                    index,
                    value: chars[index+1].to_string()
                });
                index += 1;
            },
            TokenType::Param |
            TokenType::WildCard => {
                let (name, inc) = name(index);
                index = inc;
                tokens.push_back(Token {
                    token_type: token,
                    index,
                    value: name
                });
            },
            _ => {
                tokens.push_back(Token {
                    token_type: TokenType::Char,
                    index,
                    value: value.to_string()
                });
                index += 1
            }
        }
        
    }

    tokens.push_back(Token {
                    token_type: TokenType::End,
                    index,
                    value: String::new()
                });

    Iter::new(tokens)
}

pub(crate) struct Iter {
    pub tokens: VecDeque<Token>
}

impl Iter {
    pub(crate) fn new(tokens: VecDeque<Token>)-> Iter {
        Iter { tokens }
    }

    pub(crate) fn peek(&self)->Option<&Token>{
        self.tokens.get(0)
    }

    pub fn try_consume(&mut self, token:TokenType)->Option<String>{
        match self.peek() {
            Some(value) => {
                if value.token_type.eq(&token) {
                    Some(self.tokens.pop_front().unwrap().value)
                } else {
                    None
                }
            },
            None => None
        }
    }

    pub fn consume(&mut self, token:TokenType)->String {
        match self.peek() {
            Some(value) => {
                if value.token_type.eq(&token) {
                    self.tokens.pop_front().unwrap().value
                } else {
                    panic!(
                        "Unexpected {} at {}, expected {}!",
                        value.token_type.as_str(),
                        value.index,
                        token.as_str()
                    )
                }
            },
            None => {
                panic!( "Unexpected End Occured!")
            }
        }
    }

    fn next(&mut self)->Option<String> {
        match self.try_consume(TokenType::Char) {
            Some(value) => Some(value),
            None =>{
                self.try_consume(TokenType::Escaped)
            }
        }
    }

    pub fn text(&mut self)->String {
        let mut result = String::new();

        while let Some(next) = self.next() {
            result += &next
        }

        result
    }

    pub fn parse(&mut self, end:TokenType) -> VecDeque<Segment> {
        let mut tokens: VecDeque<Segment> = VecDeque::new();

        loop {
            let path = self.text();
            if !path.is_empty() {
                tokens.push_back(Segment::Text(path));
            }

            let param = self.try_consume(TokenType::Param);
            if param.is_some() {
                tokens.push_back(Segment::Parameter(param.unwrap()));
                continue;
            }
            
            let wildcared = self.try_consume(TokenType::WildCard);
            if wildcared.is_some() {
                tokens.push_back(Segment::WildCard(param.unwrap()));
                continue;
            }
            
            let open = self.try_consume(TokenType::OpenBracket);
            if open.is_some() {
                tokens.push_back(Segment::Group(self.parse(TokenType::ClosedBracket)));
                continue;
            }

            self.consume(end);
            return tokens;
        }
    }
}

pub(crate) enum Segment{
    Parameter(String),
    WildCard(String),
    Text(String),
    Group(VecDeque<Segment>)
}

pub(crate) enum Flattened {
    Parameter(String),
    WildCard(String),
    Text(String)
}

pub(crate) fn flatten(segments:&mut VecDeque<Segment>) -> VecDeque<Flattened> {
    let mut output:VecDeque<Flattened> = VecDeque::new();

    while let Some(value) = segments.pop_front() {
        match value {
            Segment::Group(mut list) => {
                output.append(&mut flatten(&mut list));
            },
            Segment::WildCard(str) => output.push_back(
                Flattened::WildCard(str)
            ),
            Segment::Parameter(str) => output.push_back(
                Flattened::Parameter(str)
            ),
            Segment::Text(str) => output.push_back(
                Flattened::Text(str)
            )
        }
    }
    
    output
}

fn escape<'a>(str: &'a str) -> String {
    return ESCAPE.replace_all(str, "\\$&").to_string()
}

fn negate<'a>(str: &'a str) -> String {
    if str.len() < 2 {
        format!("[^{}]", DELIMITER.to_owned()+str)
    } else {
        format!(
            "(?:(?!{})[^{}])",
            escape(str),
            &ESCAPED_DELIMITER
        )
    }
}

//toRegExp
pub fn compile(path:&String, trailing:bool, end:bool) -> (String, Vec<String>){
    let mut iter = lexer(path);
    let mut tokens = iter.parse(TokenType::End);
    let mut segments = flatten(&mut tokens);
    
    let mut pattern:String = String::new();
    let mut keys:Vec<String> = Vec::new();

    let mut backtrack:String = String::new();
    let mut is_safe_segment_param:bool = true;

    while let Some(seg) = segments.pop_front() {
        match seg {
            Flattened::Text(value) => {
                pattern += &escape(&value);
                backtrack += &value;
                is_safe_segment_param = is_safe_segment_param || value.contains(DELIMITER);
            },
            Flattened::WildCard(value) => {
                if !is_safe_segment_param && backtrack.is_empty() {
                    panic!("Missing text after '${}'!", value)
                }
                pattern += "([\\s\\S]+)";
                keys.push(value);
                is_safe_segment_param = false;
            },
            Flattened::Parameter(value) => {
                if !is_safe_segment_param && backtrack.is_empty() {
                    panic!("Missing text after '${}'!", value)
                }
                pattern += &format!("({})+",
                    negate( if is_safe_segment_param {
                        EMPTY_STRING
                    } else {
                        &backtrack
                    })
                );
                keys.push(value);
                is_safe_segment_param = false;
            }
        }
    }

    pattern = format!("^(?:{})", pattern);

    if trailing {
        pattern += &format!("(?:${}$)?", ESCAPED_DELIMITER);
    }

    if end {
        pattern += &String::from("$");
    } else {
        pattern += &format!("(?={}|$)", ESCAPED_DELIMITER);
    }

    (
        pattern,
        keys
    )
}