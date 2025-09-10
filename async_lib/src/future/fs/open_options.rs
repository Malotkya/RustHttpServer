use super::{canonicalize, File};
use std::{
    io,
    path::Path,
    sync::Arc
};

pub struct OpenOptions {
    inner: std::fs::OpenOptions
}

impl OpenOptions {
    pub fn new() -> Self {
        Self { inner: std::fs::OpenOptions::new() }
    }

    pub fn read(&mut self, value:bool) -> &mut Self {
        self.inner.read(value);
        self
    }

    pub fn write(&mut self, value:bool) -> &mut Self {
        self.inner.write(value);
        self
    }

    pub fn append(&mut self, value:bool) -> &mut Self {
        self.inner.append(value);
        self
    }

    pub fn truncate(&mut self, value:bool) -> &mut Self {
        self.inner.truncate(value);
        self
    }

    pub fn create(&mut self, value:bool) -> &mut Self {
        self.inner.create(value);
        self
    }

    pub fn create_new(&mut self, value:bool) -> &mut Self {
        self.inner.create_new(value);
        self
    }

    pub async fn open<P: AsRef<Path>>(self, path: P) -> io::Result<File> {
        let data = canonicalize(path)?;
        let inner = Arc::new(self.inner);

        let file = crate::thread_await(move ||{
            inner.open(data.clone())
        }).await?;

        Ok(File {
            io: Arc::new(file)
        })
    }
}