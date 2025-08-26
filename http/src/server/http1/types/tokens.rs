use std::{collections::VecDeque, fmt};

pub trait Tokenizer: Sized {
    fn as_str<'a>(&'a self) -> &'a str;
    fn len(&self) -> usize;
    fn at(&self, index:usize) -> Option<&u8>;
    fn ptr(&self, index:usize) -> *mut u8;
    fn split<'a>(&'a self) -> SplitIterator<'a, Self>;
    fn tokenize<'a>(&'a self) -> TokenIterator<'a, Self>;
}

macro_rules! tokenizer_iterator {
    ($name:ident) => {
        pub struct $name<'a, T> where T: Tokenizer {
            ptr:&'a T,
            index: usize,
            text_start: usize      
        }

        impl<'a, T> $name<'a, T> where T: Tokenizer {
            pub fn new(tokenizer:&'a T) -> Self {
                Self {
                    ptr: tokenizer,
                    index: 0,
                    text_start: 0,
                }
            }
        }

        tokenizer_iterator!(private $name);
    };
    ($name:ident back_log=$item:ty) => {
        pub struct $name<'a, T> where T: Tokenizer {
            ptr:&'a T,
            index: usize,
            text_start: usize,
            back_log: VecDeque<$item>
        }

        impl<'a, T> $name<'a, T> where T: Tokenizer {
            pub fn new(tokenizer:&'a T) -> Self {
                let len = tokenizer.len();
                Self {
                    ptr: tokenizer,
                    index: 0,
                    text_start: 0,
                    back_log: VecDeque::with_capacity(if len < 2 {
                        1
                    } else {
                        len / 2
                    })
                }
            }
        }

        tokenizer_iterator!(private $name);
    };
    (private $name:ident) => {
        impl<'a, T> $name<'a, T> where T:Tokenizer {
            fn peek(&self) -> Option<&u8> {
                self.ptr.at(self.index)
            }

            fn get_text(&mut self) -> Option<Text> {
                if self.text_start < self.index {
                    let t = Text{
                        ptr: self.ptr.ptr(self.index),
                        size: self.index - self.text_start
                    };
                    self.text_start = self.index;
                    Some(t)
                } else {
                    None
                }
            }
        }
    };
}

tokenizer_iterator!(SplitIterator);
impl<'a, T> Iterator for SplitIterator<'a, T> where T: Tokenizer{
    type Item = Text;

    fn next(&mut self) -> Option<Text> {
        while let Some(value) = self.peek() {
            if *value < 32 || *value > 126 {
                let t = self.get_text();
                self.index += 1;
                return t;
            } else {
                self.index += 1;
            }
        }

        self.get_text()
    }
}

tokenizer_iterator!(TokenIterator back_log=Tokens);
impl<'a, T> Iterator for TokenIterator<'a, T> 
    where T: Tokenizer {
    type Item = Tokens;

    fn next(&mut self) -> Option<Self::Item> {
        if self.back_log.len() > 0 {
            return self.back_log.pop_front();
        }

        while let Some(value) = self.peek() {
            let t = if *value < 32 || *value > 126 {
                self.get_text()
            } else if let Some(sep) = Seperator::from(*value as char) {
                self.back_log.push_back(Tokens::Seperator(sep));
                self.get_text()
            } else {
                None
            };

            self.index += 1;

            if t.is_some() {
                return Some(Tokens::Text(t.unwrap()));
            } else {
                return self.back_log.pop_front();
            }
        }

        match self.get_text() {
            Some(t) => Some(
                Tokens::Text(t)
            ),
            None => None
        }
    }
}

#[derive(Clone)]
pub enum Tokens {
    Seperator(Seperator),
    Text(Text),
}

#[derive(Debug, Clone)]
pub enum TokenError {
    MismatchError(Seperator, Seperator),
    SeperatorError(Option<Seperator>, Text),
    TextError(Option<&'static str>, Seperator)
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchError(exp, act) => write!(
                f,
                "Expected seperator '{}' but instead found '{}'!",
                Into::<char>::into(*exp),
                Into::<char>::into(*act)
            ),
            Self::SeperatorError(Some(exp), act) => write!(
                f,
                "Expected seperator '{}' but instead found \"{}\"!",
                Into::<char>::into(*exp),
                act.as_str()
            ),
            Self::SeperatorError(None, act) => write!(
                f,
                "Expected seperator but instead found \"{}\"!",
                act.as_str()
            ),
            Self::TextError(name, act ) => write!(
                f,
                "Expected {} but instead found '{}'!",
                name.unwrap_or("text"),
                Into::<char>::into(*act)
            )
        }
    }
}

#[allow(dead_code)]
impl Tokens {
    pub fn is_seperator(&self) -> bool {
        match self {
            Self::Seperator(_) => true,
            Self::Text(_) => false
        }
    }

    pub fn seperator(&self, expect:Option<Seperator>) -> Result<Seperator, TokenError> {
        match self {
            Self::Seperator(sep) => {
                if expect.is_none() {
                    Ok(*sep)
                } else {
                    let exp = expect.unwrap();
                    if *sep == exp  {
                        Ok(*sep)
                    } else {
                        Err(TokenError::MismatchError(exp, *sep))
                    }
                }
            },
            Self::Text(txt) => Err(TokenError::SeperatorError(expect, txt.clone()))
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Self::Seperator(_) => false,
            Self::Text(_) => true
        }
    }

    pub fn text(&self, name:Option<&'static str>) -> Result<Text, TokenError> {
        match self {
            Self::Seperator(sep) => Err(TokenError::TextError(name, *sep)),
            Self::Text(txt) => Ok(txt.clone())
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Seperator {
    OpenParenthesis,
    CloseParenthesis,
    OpenAngleBracket,
    CloseAngleBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    At,
    Comma,
    DoubleQuotes,
    QuestionMark,
    Equals,
    Colon,
    Semicolon,
    ForwardSlash,
    BackSlash
}

impl Seperator {
    fn from(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::OpenParenthesis),
            ')' => Some(Self::CloseParenthesis),
            '<' => Some(Self::OpenAngleBracket),
            '>' => Some(Self::CloseAngleBracket),
            '{' => Some(Self::OpenCurlyBracket),
            '}' => Some(Self::CloseCurlyBracket),
            '[' => Some(Self::OpenSquareBracket),
            ']' => Some(Self::CloseSquareBracket),
            '@' => Some(Self::At),
            ',' => Some(Self::Comma),
            '"' => Some(Self::DoubleQuotes),
            '?' => Some(Self::QuestionMark),
            '=' => Some(Self::Equals),
            ':' => Some(Self::Colon),
            ';' => Some(Self::Semicolon),
            '/' => Some(Self::ForwardSlash),
            '\\' => Some(Self::BackSlash),
            _ => None
        }
    }
}

impl Into<char> for Seperator {
    fn into(self) -> char {
        match self {
            Self::OpenParenthesis    => '(',
            Self::CloseParenthesis   => ')',
            Self::OpenAngleBracket   => '<',
            Self::CloseAngleBracket  => '>',
            Self::OpenCurlyBracket   => '{',
            Self::CloseCurlyBracket  => '}',
            Self::OpenSquareBracket  => '[',
            Self::CloseSquareBracket => ']',
            Self::At                 => '@',
            Self::Comma              => ',',
            Self::DoubleQuotes       => '"',
            Self::QuestionMark       => '?',
            Self::Equals             => '=',
            Self::Colon              => ':',
            Self::Semicolon          => ';',
            Self::ForwardSlash       => '/',
            Self::BackSlash          => '\\',
        }   
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Text {
    ptr: *const u8,
    size: usize
}

impl ToString for Text {
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

impl Tokenizer for Text {
    fn as_str<'a>(&'a self) -> &'a str {
        unsafe { std::str::from_raw_parts(self.ptr, self.size) }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn at(&self, index:usize) -> Option<&u8> {
        if index > self.size {
            None
        } else {
            unsafe {
                Some(&*self.ptr(index))
            }
        }
    }

    fn ptr(&self, index:usize) -> *mut u8 {
        unsafe {self.ptr.byte_offset(index as isize) as *mut u8}
    }

    fn split<'a>(&'a self) -> SplitIterator<'a, Self> {
        SplitIterator::new(self)
    }

    fn tokenize<'a>(&'a self) -> TokenIterator<'a, Self> {
        TokenIterator::new(self)
    }
}

