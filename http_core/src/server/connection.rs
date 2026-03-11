use async_lib::{
    io::Result,
    net::TcpStream
};
use crate:: {
    server::{
        Server,
        error::write_connection_error_response,
        request::build_request,
        response::write_response
    },
    response::Response,
    error::{HttpError, HttpErrorKind}
};


pub(crate) async fn build_connections(mut stream:TcpStream) -> std::io::Result<(TcpStream, TcpStream)> {
    let response = match stream.try_clone() {
        Ok(stream) => stream,
        Err(e) => {
            write_connection_error_response(
                &mut stream,
                Response::from_error(
                    HttpError::new(
                        HttpErrorKind::InternalServerError,
                        "An error occured while handeling the connection!"
                    )
                )
            ).await?;
            return Err(e);
        }
    };

    Ok((stream, response))
}

pub(crate) async fn handle_connection<S: Server>(server:S, stream:TcpStream) -> Result<()> {
    let (req_stream, mut resp_stream) = build_connections(stream).await?;

    if let Some(mut request) = build_request(req_stream, &mut resp_stream, server.hostname(), server.port()).await? {
        let response = server.handle_request(&mut request).await;
        //TODO!! EVENT crate::log(&request, &response);
        write_response(
            &mut resp_stream,
            response,
            request.version
        ).await?;
    }

    Ok(())
}