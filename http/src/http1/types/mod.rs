use std::{collections::LinkedList, fmt, io::{BufRead, BufReader, Read} };

mod tokens;
pub use tokens::*;
mod version;
pub use version::parse_version;
mod uri;
pub use uri::{Uri, UriError};

const CHUNK_SEPERATOR: &'static [u8] = b"\r\n";

#[derive(Debug)]
pub enum ParseStreamError {
    ReadError(std::io::Error),
    BufferTaken,
    ParseError(usize)
}

impl fmt::Display for ParseStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadError(e) => write!(f, "{}", e),
            Self::BufferTaken => write!(f, "Stream Buffer is taken!"),
            Self::ParseError(index) => write!(f, "Invalid character found at: {index}!")
        }
    }
}

fn next_chunk<'a>(buffer:&'a [u8], mut index:usize) -> Result<Option<usize>, ParseStreamError> {
    if buffer[index] > 127 {
        return Err(ParseStreamError::ParseError(index));
    }
    
    let length = buffer.len();
    while index < length {
        let next = index+1;
        if buffer[next] > 127 {
            return Err(ParseStreamError::ParseError(next));

        } else if buffer[index..=next] == *CHUNK_SEPERATOR {
            return Ok(Some(index));
        }

        index = next;
    }

    Ok(None)
}

pub struct StreamParser<S> where S: Read{
    reader: Option<S>,
    buffer: LinkedList<Chunk>,
}

impl<S> StreamParser<S>  where S: Read{
    pub fn new(stream:S) -> Self {
        Self {
            reader: Some(stream),
            buffer: LinkedList::new()
        }
    }

    pub fn take_reader(&mut self) -> Option<BufReader<S>> {
        self.reader.take().map(|s|BufReader::new(s))
    }

    pub fn parse(&mut self) -> Result<Option<Chunk>, ParseStreamError> {
        if self.reader.is_none() {
            return Err(ParseStreamError::BufferTaken);
        }

        let next = self.buffer.pop_front();
        if next.is_some() {
            return Ok(next);
        }
        let mut reader = BufReader::new(self.reader.as_mut().unwrap());
        let mut index: usize = 0;
        let sep_len = CHUNK_SEPERATOR.len();
        let peek_buffer = reader.fill_buf()
            .map_err(|e|ParseStreamError::ReadError(e))?;

        if peek_buffer.len() == 0 {
            return Ok(None);
        }

        while let Some(next) = next_chunk(peek_buffer, index)? {
            let slice = &peek_buffer[index..next];
            self.buffer.push_back(Chunk(Vec::from(slice)));

            index = next + sep_len;
        }
        reader.consume(index);

        Ok(self.buffer.pop_front())
    }
}

pub struct Chunk(Vec<u8>);

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
    pub fn has_some(&self) -> bool {
        !self.0.is_empty()
    }
}





