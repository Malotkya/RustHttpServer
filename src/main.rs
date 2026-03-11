pub use http::{
    types::{Request, Response, ErrorRequest},
    builder::{router, server},
    types::*
};

#[router(path="/Hello/:Name")]
async fn TestName(req: Request<TestNamePathParam>) -> http::Result {
    Ok(Response::from(format!("Hello {}!", req.param.Name)))
}

#[router(path="/")]
async fn Home(_: Request<HomePathParam>) -> http::Result {
    Ok(Response::from("Hello World!"))
}

async fn error_handler(mut req:ErrorRequest) -> Response {
    req.param.message = "You done messed up!".to_string();
    Response::from_error(req.param)
}

#[server]
pub struct ServerName ( 
    Home,
    TestName,
    error_handler
);

#[cfg(test)]
mod test {
    use http::builder::Server;
    use super::*;

    #[test]
    fn test_default_override() {
        let s = ServerName::new(Some("localhost".to_string()), Some(8080));
        
        assert_eq!(
            s.hostname(),
            "localhost"
        );

        assert_eq!(
            s.port(),
            8080
        );
    }

    #[test]
    fn debug_server() {
        ServerName::new(None, None)
            .start(1).unwrap()
    }
}