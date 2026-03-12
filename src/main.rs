pub use http::{
    types::*,
    server, router
};

#[router(path="/Hello/:Name")]
async fn TestName(req: Request<TestNamePathParam>) -> Result<Response> {
    Response::from(format!("Hello {}!", req.param.Name)).send()
}

#[router(path="/")]
async fn Home(_: Request<HomePathParam>) -> Result<Response> {
    Response::from("Hello World!").send()
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
    use http::server::{Server, ServerOpts};
    use super::*;

    #[test]
    fn test_default_override() {
        let s = ServerName::new(ServerOpts::new(
            "localhost",
            8080,
            20
        ));
        
        assert_eq!(
            s.hostname(),
            "localhost"
        );

        assert_eq!(
            s.port(),
            8080
        );

        assert_eq!(
            s.threads(),
            20
        )
    }

    #[test]
    fn debug_server() {
        ServerName::new(ServerOpts::threads(1))
            .start().unwrap()
    }
}