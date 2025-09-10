use http_types::{Version, Response};
use async_lib::io::{AsyncWrite, Result};

pub async fn write_response<S>(resp:Response, ver:Version, stream:&mut S) -> Result<()> where S: AsyncWrite {
    stream.write(format!(
        "{} {} {}\r\n",
        ver.to_string(),
        resp.status.code().to_string(),
        resp.status.as_str()
    ).as_bytes()).await?;

    for (key, value) in resp.headers.into_iter() {
         stream.write(&format!(
            "{}: {}\r\n",
            key.name(),
            value.ref_str().unwrap()
        ).as_bytes()).await?;
    }

    stream.write(b"\r\n").await?;

    for chunk in resp.body {
        stream.write(chunk.value()).await?;
    }

    Ok(())
}