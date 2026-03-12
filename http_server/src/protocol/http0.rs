

use async_lib::io::{ AsyncRead, AsyncWrite, Result };
use http_core::{
    headers::Headers,
    method::Method,
    url::Url,
    version::Version,
    request::RequestBuilder,
    response::Response
};
use super::{
    BuildError,
    types::Uri
};


/// Http/0.9 Request Format:
/// 
/// GET [PATH]
/// 
pub fn build_request<S>(port:u16, method: Method, path:Uri) -> std::result::Result<RequestBuilder<S>, BuildError> 
    where S: AsyncRead {
    if method != Method::GET {
        return Err(BuildError::Http0GetMethodOnly)
    }

    let path = match path.absolute_path() {
        Ok(str) => str,
        Err(_) => return Err(BuildError::Http0AbsolutePathOnly)
    };

    Ok(RequestBuilder::new(
        Url::empty(port, &path),
        method,
        Headers::new(),
        Version{major: 0, minor: 9},
        None
    ))
}



pub async fn write_response<S>(resp: Response, stream:&mut S) -> Result<()> where S: AsyncWrite {
    for chunk in resp.body {
        stream.write(chunk.value()).await?;
    }
    
    Ok(())
}