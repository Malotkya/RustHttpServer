
pub mod error;
pub mod headers;
pub mod method;
pub mod path;
pub mod request;
pub mod response;
pub mod status;
pub mod url;

pub mod result {
    use super::error::ValidHttpError;

    pub type Result<T> = std::result::Result<T, Box<dyn ValidHttpError>>;
}

pub mod version {
    pub struct Version {
        pub major: u8,
        pub minor: u8
    }

    impl ToString for Version {
        fn to_string(&self) -> String {
            format!("HTTP/{}.{}", self.major, self.minor)
        }
    }
}


