pub use http_macro::{router, server};

#[router(path="/Hello/:Name")]
async fn TestName<'b>(req: http::Request<'b, impl std::io::Read, TestNamePathParam<'b>>) -> Result<http::Response, http::HttpError> {
    Ok(http::Response::from(format!("Hello {}!", req.param.Name)))
}

#[server(5000)]
struct ServerName (
    TestName
);


fn main() {
    let server = ServerName::connect(ServerNameParts::new()).unwrap();
    let mut exec = http::Executor::new();
    loop {
        if let Some(task) = server.next().unwrap() {
            exec.spawn(task);
        }

        exec.run_ready_tasks();
        //exec.sleep_if_idle();
    }
}
 