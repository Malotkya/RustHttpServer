use http_types::Response;
use async_lib::io::{AsyncWrite, Result};

pub async fn write<S>(resp: Response, stream:&mut S) -> Result<()> where S: AsyncWrite {
    for chunk in resp.body {
        stream.write(chunk.value()).await?;
    }
    
    Ok(())
}