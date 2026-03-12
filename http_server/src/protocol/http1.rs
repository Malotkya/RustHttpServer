/// Following RFC-2616 Stanard:
/// https://datatracker.ietf.org/doc/html/rfc2616
/// 
use async_lib::io::{AsyncRead, AsyncWrite, Result};
use http_core::{
    method::Method,
    headers::Headers,
    url::ToUrl,
    version::Version,
    response::Response,
    request::RequestBuilder,
};
use super::{
    BuildError,
    types::*
};




/// http/1.1 Request Format:
/// 
/// [METHOD] %SP% [Request-URI] %SP% [HTTP-VERSION] %CRLF%
/// ([REQUEST_HEADER_NAME] ":" [REQEUST_HEADER_VALUE] %CRLF%
/// %CRLF%
/// [BODY]
/// 
pub async fn build_request<S>(stream:S, hostname:&str, port:u16) -> std::result::Result<RequestBuilder<S>, BuildError>
    where S: AsyncRead {

    let mut parser = StreamParser::new(stream);

    let start_line = match parser.parse().await {
        Ok(Some(line)) => line,
        Ok(None) => return Err(BuildError::EmptyRequest),
        Err(e) => match e {
            ParseStreamError::ReadError(e) =>
                return Err(BuildError::IoError(e)),
            parse_err => {
                return Err(BuildError::ParseError(parse_err));
            }
         }
    };

    let mut it = start_line.split();
    let method = match  it.next() {
        Some(t) => match Method::from(t.as_str()) {
            Some(m) => m,
            None => return Err(
                BuildError::InvalidMethod(t.decode())
            )
        },
        None => return Err(BuildError::MissingMethod)
    };

    let uri = match it.next() {
        Some(t) => match Uri::parse(&t) {
            Ok(uri) => uri,
            Err(e) => return Err(
                BuildError::InvalidUri(e)
            )
        },
        None => return Err(
            BuildError::MissingUri
        )
    };

    let version = match it.next() {
        Some(t) => match parse_version(&t) {
            Ok(v) => v,
            Err(_) => return Err(
                BuildError::InvalidVersion(t.decode())
            )
        }
        None => return Err(
            BuildError::MissingVersion(method, uri)
        )
    };

    let mut headers = Headers::new();
    while let Some(chunk) = parser.parse().await.map_err(|e|match e{
        ParseStreamError::ReadError(e) => BuildError::IoError(e),
        parse_err => BuildError::ParseError(parse_err)
    })? && chunk.has_some() {
        //chunk = Header Name: Header Value
        let mut line = chunk.as_str().split(':');

        headers.set(
            line.next().unwrap().trim(),
            line.next().unwrap_or("").trim()
        );
    }

    Ok(
        RequestBuilder::new(
            uri.to_url(hostname.into(), port)
                    .map_err(|e|BuildError::InvalidUrl(e))?,
            method,
            headers,
            version,
            Some(parser.take_reader().unwrap())
        )
    )
}

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