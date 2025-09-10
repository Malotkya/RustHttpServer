use super::canonicalize;
use std::{
    io,
    path::Path,
    sync::Arc
};

pub struct DirBuilder {
    io: std::fs::DirBuilder
}

impl DirBuilder {
    pub fn new() -> Self {
        Self{ 
            io: std::fs::DirBuilder::new()
        }
    }

    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.io.recursive(recursive);
        self
    }

    pub async fn create<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        let data = canonicalize(path)?;
        let inner = Arc::new(self.io);
        crate::await_thread(move ||{
            inner.create(data.clone())
        }).await
    }
}