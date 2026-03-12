pub use types::{
    Response,
    Request,
    Result,
    HttpError,
    HttpErrorKind
};
pub use server::{
    server,
    router::router
};

pub mod json {
    pub use util::json::{
        JsonError, JsonRef, JsonValue
    };
}

pub mod types {
    pub use http_core::{
        error::{HttpErrorKind, HttpError, ValidHttpError},
        method::Method,
        request::{Request, ErrorRequest},
        response::Response,
        status::HttpStatus,
        url::{Hostname, Url, ToUrl},
        headers,
        version::Version,
        result::Result
    };

    pub mod response {
        pub use http_core::response::Chunk;
    }
}

pub mod server {
    pub use http_server::*;
    pub use http_core::request::RequestBuilder;

    pub use async_lib::executor;
}

pub mod async_net {
    pub use async_lib::net::*;
}

pub mod async_io {
    pub use async_lib::io::*;
}

pub mod async_fs {
    pub use async_lib::fs::*;
}

pub mod event {
    pub use async_lib::{
        EventEmitter, EventEmitterWrapper
    };
}

pub mod promise {
    pub use async_lib::{
        promise, Promise,
    };
}

pub mod html {
    pub use html::*;
}

/*pub(crate) fn log(req: &RequestBuilder<impl AsyncRead>, resp: &types::Response) {
    println!("{:?} {:?}", req, resp);
}*/
