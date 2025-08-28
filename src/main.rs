pub use http_macro::{router, server};

#[router(path="/Hello/:Name")]
async fn TestName(req: http::Request<TestNamePathParam>) -> http::Result {
    Ok(http::Response::from(format!("Hello {}!", req.param.Name)))
}

#[router(path="/")]
async fn Home(_: http::Request<HomePathParam>) -> http::Result {
    Ok(http::Response::from("Hello World!"))
}

async fn error_handler(_:http::ErrorRequest) -> http::Response {
    http::Response::from("You done messed up!")
}

#[server(port=8080, is_async=true)]
struct ServerName ( 
    Home,
    TestName,
    error_handler
);

fn main() {
    ServerName::start()
        .expect("There was a problem starting the server!");
}
 