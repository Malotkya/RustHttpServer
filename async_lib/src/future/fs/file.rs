use std::{
    path::{Path, PathBuf},
    io,
};
use async_lib_macros::deref_inner_async;
use crate::thread_await;

#[deref_inner_async(Read, Write)]
pub struct File {
    io: std::fs::File
}

impl File {
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let data = PathBuf::from(path.as_ref());

        let file = thread_await(move ||{
            std::fs::File::open(data.clone())
        }).await?;
        
        Ok(Self{
            io: file
        })
    }
}