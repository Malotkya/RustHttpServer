pub trait Tokenizer: Sized {
    fn as_str<'a>(&'a self) -> &'a str;
    fn len(&self) -> usize;
    fn at(&self, index:usize) -> Option<&u8>;
    fn ptr(&self, index:usize) -> *mut u8;
}

pub fn split<T:Tokenizer>(value:T) -> Vec<Text> {
    let mut vec: Vec<_> = Vec::new();
    let mut start:usize = 0;
    let mut i:usize = 0;

    let len = value.len();
    while i < len {
        let char = value.at(i).unwrap();

        if *char < 32 || *char > 126 {
            if start < i {
                vec.push(Text {
                    ptr: value.ptr(i),
                    size: i - start
                });
            }
            start = i;
        }

        i += 1
    }

    if start < i {
        vec.push(Text {
            ptr: value.ptr(i),
            size: i - start
        });
    }

    vec
}

pub fn tokenize<T:Tokenizer>(value:&T) -> Vec<Tokens> {
    let mut vec: Vec<_> = Vec::new();
    let mut start:usize = 0;
    let mut i:usize = 0;

    let len = value.len();
    while i < len {
        let char = value.at(i).unwrap();

        if *char < 32 || *char > 126 {
            if start < i {
                vec.push(Tokens::Text(
                    Text {
                        ptr: value.ptr(i),
                        size: i - start
                    }
                ));
            }
            start = i;
        } else if let Some(sep) = Seperator::from(*char as char) {
            if start < i {
                vec.push(Tokens::Text(
                    Text {
                        ptr: value.ptr(i),
                        size: i - start
                    }
                ));
            }
            start = i;
            vec.push(Tokens::Seperator(
                sep
            ));
        }

        i += 1;
    }
    
    if start < i {
        vec.push(Tokens::Text(
            Text {
                ptr: value.ptr(i),
                size: i - start
            }
        ));
    }

    vec
}

#[derive(Eq, PartialEq, Clone, Copy)]
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

#[derive(PartialEq, Eq, Clone)]
pub struct Text {
    ptr: *const u8,
    size: usize
}

impl Text {
    pub fn from(vec:&Vec<u8>) -> Self {
        Self { 
            ptr: vec.as_ptr(),
            size: vec.len()
        }
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
}

pub enum Tokens {
    Seperator(Seperator),
    Text(Text),
}