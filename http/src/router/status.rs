use crate::error::HttpError;
use std::io::Result;

type Continue = Option<HttpError>;
pub type Status = Result<Continue>;

pub fn next()->Status {
    Ok(None)
}

pub fn error(err: HttpError)->Status{
    Ok( Some(err) )
}
