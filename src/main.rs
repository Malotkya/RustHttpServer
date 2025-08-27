pub use http_macro::{router, server};

#[router(path="/Hello/:Name")]
async fn TestName(req: http::Request<TestNamePathParam>) -> Result<http::Response, http::HttpError> {
    Ok(http::Response::from(format!("Hello {}!", req.param.Name)))
}

#[router(path="/")]
async fn Home(_: http::Request<HomePathParam>) -> http::Result {
    Ok(http::Response::from("Hello World!"))
}

#[server(5000)]
struct ServerName (
    Home,
    TestName
);


fn main() {
    let server = ServerName::connect(ServerNameParts::new()).unwrap();
    let mut exec = http::Executor::new();

    println!("Listening at: http://{}", server.to_string());

    loop {
        if let Some(task) = server.next().unwrap() {
            exec.spawn(task);
        }

        exec.run_ready_tasks();
        //exec.sleep_if_idle();
    }
}
 