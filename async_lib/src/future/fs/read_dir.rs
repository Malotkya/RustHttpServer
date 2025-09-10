use std::{
    async_iter::AsyncIterator,
    path::Path,
    io
};
use super::canonicalize;

pub struct ReadDir {
    inner: std::fs::ReadDir
}

impl AsyncIterator for ReadDir {
    type Item = io::Result<super::DirEntry>;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Ready(self.inner.next())
    }
}

pub async fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
    let data = canonicalize(path)?;

    let reader = crate::await_thread(move||{
        std::fs::read_dir(data.clone())
    }).await?;

    Ok(ReadDir{
        inner: reader
    })
}