use std::{io::Read, net::TcpStream, fmt::Display};

mod tokens;
pub use tokens::*;
mod version;
pub use version::parse_version;
mod uri;
pub use uri::{Uri, UriError};

const CHUNK_SIZE:usize = 1024;

pub enum ParseStreamError<ParseError>
    where ParseError: Display{
    ReadError(std::io::Error),
    ParseError(ParseError)
}

pub trait ParseStream: Sized{
    type Error: Display;
    type Type: Tokenizer;
    fn parse(&mut self) -> Result<Option<Self::Type>, ParseStreamError<Self::Error>>;
}


pub struct TcpStreamParser(pub TcpStream);

impl TcpStreamParser {
    pub fn new(stream:TcpStream) -> Self {
        Self(stream)
    }
}

impl ParseStream for TcpStreamParser {
    type Error = &'static str;
    type Type = Chunk;

    fn parse(&mut self) -> Result<Option<Self::Type>, ParseStreamError<Self::Error>> {
        let mut chunk:Chunk = Chunk(Vec::with_capacity(CHUNK_SIZE));

        loop {
            let mut buffer:Vec<u8> = Vec::with_capacity(CHUNK_SIZE);

            match self.0.peek(&mut buffer) {
                Ok(0) => {
                    return Ok(None);
                },
                Ok(size) => {
                    buffer.truncate(size);
                    chunk.0.append(&mut buffer);
                    match chunk.format() {
                        Some(size) => {
                            self.0.read(&mut Vec::with_capacity(size));
                            return Ok(Some(chunk));
                        },
                        None => {
                            continue;
                        }
                    }
                },
                Err(e) => return Err(ParseStreamError::ReadError(e))
            }
        }
    }
}

struct Chunk(Vec<u8>);

impl Tokenizer for Chunk {
    fn as_str<'a>(&'a self) -> &'a str {
        unsafe {
            std::str::from_raw_parts(self.0.as_ptr(), self.0.len())
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn at(&self, index:usize) -> Option<&u8> {
        self.0.get(index)
    }

    fn ptr(&self, index:usize) -> *mut u8 {
        unsafe { self.0.as_ptr().byte_offset(index as isize) as *mut u8 } 
    }

    fn split<'a>(&'a self) -> SplitIterator<'a, Self> {
        SplitIterator::new(self)
    }

    fn tokenize<'a>(&'a self) -> TokenIterator<'a, Self> {
        TokenIterator::new(self)
    }
}

impl Chunk {
    fn format(&mut self) -> Option<usize> {
        match self.as_str().find("\r\n") {
            Some(index) => {
                self.0.truncate(index);
                Some(index)
            },
            None => None
        }
    }

    pub fn has_some(&self) -> bool {
        !self.0.is_empty()
    }
}





