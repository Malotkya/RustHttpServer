use http_types::{Version, Response};
use std::io::{Write, Result};

pub fn write_response<S>(resp:Response, ver:Version, stream:&mut S) -> Result<()> where S: Write {
    stream.write(format!(
        "{} {} {}\r\n",
        ver.to_string(),
        resp.status.code().to_string(),
        resp.status.as_str()
    ).as_bytes())?;

    for (key, value) in resp.headers.into_iter() {
         stream.write(&format!(
            "{}: {}\r\n",
            key.name(),
            value.ref_str().unwrap()
        ).as_bytes())?;
    }

    stream.write(b"\r\n")?;

    for chunk in resp.body {
        stream.write(chunk.value())?;
    }

    Ok(())
}