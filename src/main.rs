pub use http_macro::{router, server};
use async_lib::executor::{start_with_callback, shut_down};
use async_lib::async_fn;

#[router(path="/Hello/:Name")]
async fn TestName(req: http::Request<TestNamePathParam>) -> http::Result {
    Ok(http::Response::from(format!("Hello {}!", req.param.Name)))
}

#[router(path="/")]
async fn Home(_: http::Request<HomePathParam>) -> http::Result {
    Ok(http::Response::from("Hello World!"))
}

async fn error_handler(mut req:http::ErrorRequest) -> http::Response {
    req.param.message = "You done messed up!".to_string();
    http::Response::from_error(req.param)
}

#[server(port=8080, is_async=true)]
struct ServerName ( 
    Home,
    TestName,
    error_handler
);

fn main() {

    start_with_callback(async_fn!({
        println!("Hello World!");
        //shut_down();
    }));
}
 