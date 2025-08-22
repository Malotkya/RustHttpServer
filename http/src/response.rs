use crate::{Headers, HttpError, HttpStatus};

pub struct Response {
    pub status: HttpStatus,
    pub headers: Headers,
    body: Vec<u8>
}

#[allow(dead_code)]
impl Response {
    pub fn new<T>(body:&[u8]) -> Self  {
        Self {
            status: HttpStatus::Ok,
            headers: Headers::new(),
            body: body.into()
        }
    }

    impl write()
}