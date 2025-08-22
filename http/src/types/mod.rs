mod url;
pub use url::*;
mod method;
pub use method::*;
mod status;
pub use status::HttpStatus;
mod json;
pub use json::*;

pub type Result<T, E: std::fmt::Display> = std::result::Result<T, E>;

pub struct Version {
    pub major: u8,
    pub minor: u8
}

pub enum BodyData {
    File(FileData),
    Value(JsonValue)
}

impl BodyData {
    fn new_file(filename:&str, mimetype:&str, data:&[u8]) -> Self {
        Self::File(
            FileData {
                name: String::from(filename),
                mimetype: String::from(mimetype),
                data: data.into()
            }
        )
    }

    fn new(data:&str) -> Self {
        Self::Value(
            JsonValue::String(String::from(data))
        )
    }

    fn value<'a>(&'a self) -> Option<JsonRef<'a>> {
        match self {
            Self::Value(data) => Some(data.into()),
            Self::File(_) => None
        }
    }

    fn file<'a>(&'a self) -> Option<&'a FileData> {
        match self {
            Self::File(data) => Some(data),
            Self::Value(_) => None
        }
    }
}

pub struct FileData {
    pub name: String,
    pub mimetype: String,
    pub data: Vec<u8>
}