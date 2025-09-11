pub use http::{
    types::{Request, Response, ErrorRequest},
    builder::{router, server}
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

#[server(port=8080)]
struct ServerName ( 
    Home,
    TestName,
    error_handler
);

fn main() {
    ServerName::start(4).unwrap();
}
 