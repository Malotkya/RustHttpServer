#![feature(str_from_raw_parts)]
use http_core::{
    request::RequestBuilder,
    response::Response
};
use async_lib::{
    executor::*, net::{TcpListener, TcpStream}, io::ErrorKind
};
use arguments::*;
pub use http_macro::server;

mod arguments;
pub(crate) mod connection;
mod protocol;


pub struct ServerOpts {
    pub port:Option<u16>,
    pub hostname:Option<String>,
    pub threads:Option<usize>
}

impl ServerOpts {
    pub fn new<H:ToString>(hostname:H, port:u16, threads:usize) -> Self {
        Self {
            hostname: Some(hostname.to_string()),
            port: Some(port.into()),
            threads: Some(threads.into())
        }
    }

    pub fn hostname<Hostname:ToString>(value:Hostname) -> Self {
        Self {
            hostname: Some(value.to_string()),
            threads: None,
            port: None
        }
    }

    pub fn port(value:u16) -> Self {
        Self {
            hostname: None,
            threads: None,
            port: Some(value.into())
        }
    }

    pub fn threads(value:usize) -> Self {
        Self {
            hostname: None,
            threads: Some(value.into()),
            port: None
        }
    }

    pub fn none() -> Self {
        Self { port: None, hostname: None, threads: None }
    }
}

pub fn get_server_opts(config_filename:Option<&str>) -> std::io::Result<ServerOpts> {
    let CommandLineArguments{
        mut port,
        mut hostname,
        mut threads,
        config
    } = arguments::get_cmd_line_args();

    if let Some(filename) = config.as_deref().or(config_filename) {
        if let Some(opts) = arguments::read_config_file(filename)? {
            port = port.or(opts.port);
            hostname = hostname.or(opts.hostname);
            threads = threads.or(opts.threads);
        }
    }

    Ok(ServerOpts { port, hostname, threads })
}

pub trait Server: 'static + Sized + Sync + Send + Clone {
    fn new(opts:ServerOpts) -> Self;
    fn hostname(&self) -> &str;
    fn port(&self) -> u16;
    fn threads(&self) -> usize;

    fn handle_request(&self, req:&mut RequestBuilder<TcpStream>) -> impl Future<Output = Response>;

    fn start(&self, /*callback:Option<impl AsyncCallback<()>*/) -> std::io::Result<()> {
        let server = self.clone();
        let listener = TcpListener::bind(server.address())?;

        init_async_thread_pool(self.threads());
        let _ = queue_job(move||{
            println!("Listening at {}", server.address());

            while is_running() {
                let clone = server.clone();

                match listener.sync_accept() {
                    Ok(conn) => spawn_task(async move{
                        if let Err(e) = connection::handle_connection(clone, conn.0).await {
                            println!("Error: {}", e)
                        }
                    }),
                    Err(e) => if e.kind() != ErrorKind::WouldBlock {
                         println!("Connection Error: {}", e);
                    }
                }
            }
        });

        println!("Starting");
        start();

        println!("Good bye!");
        Ok(())
    }

    fn address(&self) -> String {
        format!("{}:{}", self.hostname(), self.port())
    }
}

pub mod router {
    use http_core::{
        response::Response,
        request::{Request, RequestBuilder},
        result::Result,
        error::HttpError
    };
    use regex::Regex;
    use std::collections::HashMap;

    pub use http_macro::{path, router};

    pub struct Path<'k, const N:usize> {
        pub regex: Regex,
        pub keys: [&'k str; N], 
    }

    impl<'k, const N:usize> Path<'k, N> {
        pub fn match_path<'a>(&self, pathname: &'a str) -> Option<HashMap<&'k str, &'a str>> {
            match self.regex.captures(pathname) {
                Some(caps) => {
                    let mut map = HashMap::new();
                    let (_, matches) = caps.extract() as (&str, [&str; N]);
                    
                    for i in 0..N {
                        map.insert(self.keys[i], matches[N]);
                    }

                    Some(map)
                },
                None => None
            }
        }
    }


    pub trait Layer<Param> {
        fn new() -> Self;
        fn match_path(&self, pathname:&str) -> Option<Param>;
        fn handler(&self, request: Request<Param>) -> impl Future<Output = Result<Response>>;
    }

    pub trait Router<Param>: Layer<Param> {

        #[allow(async_fn_in_trait)]
        async fn handle(&self, req:&mut RequestBuilder<async_lib::net::TcpStream>) -> std::result::Result<Option<Response>, HttpError> {
            match self.match_path(&req.url.pathname()) {
                Some(param) =>  self.handler(req.build(param)).await
                    .map(|resp|Some(resp)),
                None => Ok(None)
            }
        }
    }

    impl<P, R:Layer<P>> Router<P> for R {}
}