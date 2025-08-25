use crate::Response;
use std::io::{Write, Result};

pub fn write<S>(resp: Response, stream:&mut S) -> Result<()> where S: Write{
    for chunk in resp.body {
        stream.write(chunk.value())?;
    }
    Ok(())
}