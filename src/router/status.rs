use crate::error::HttpError;
use std::io::Result;

type Continue = Option<HttpError<String>>;
pub type Status = Result<Continue>;

pub fn next()->Status {
    Ok(None)
}

pub fn error(err: HttpError<String>)->Status{
    Ok( Some(err) )
}
