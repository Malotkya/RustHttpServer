use std::{
    path::Path,
    io,
    sync::Arc
};
use super::canonicalize;
use async_lib_macros::deref_inner_async;
use crate::{io::AsyncBufReader, thread_await};

#[deref_inner_async(Read, Write)]
pub struct File {
    pub(crate) io: Arc<std::fs::File>
}

impl File {
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let data = canonicalize(path)?;

        let file = thread_await(move ||{
            std::fs::File::open(data.clone())
        }).await?;
        
        Ok(Self{
            io: Arc::new(file)
        })
    }

    pub async fn open_buffered<P: AsRef<Path>>(path: P) -> io::Result<AsyncBufReader<File>> {
        let data = canonicalize(path)?;

        let file = thread_await(move ||{
            std::fs::File::open(data.clone())
        }).await?;

        Ok(
            AsyncBufReader::new(Self{
                io: Arc::new(file)
            })
        )
    }

    pub async fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let data = canonicalize(path)?;

        let file = thread_await(move ||{
            std::fs::File::create(data.clone())
        }).await?;

        Ok(Self {
            io: Arc::new(file)
        })
    }

    pub async fn create_buffered<P: AsRef<Path>>(path: P) -> io::Result<AsyncBufReader<File>> {
        let data = canonicalize(path)?;

        let file = thread_await(move ||{
            std::fs::File::create(data.clone())
        }).await?;

        Ok(
            AsyncBufReader::new(Self{
                io: Arc::new(file)
            })
        )
    }

    pub async fn create_new<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let data = canonicalize(path)?;

        let file = thread_await(move ||{
            std::fs::File::create_new(data.clone())
        }).await?;

        Ok(Self {
            io: Arc::new(file)
        })
    }

    pub async fn options() -> super::OpenOptions {
        super::OpenOptions::new()
    }

    pub async fn sync_all(&self) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.sync_all()
        }).await
    }

    pub async fn sync_data(&self) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.sync_data()
        }).await
    }

    pub async fn lock(&self) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.lock()
        }).await
    }

    pub async fn lock_shared(&self) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.lock_shared()
        }).await
    }

    pub async fn try_lock(&self) -> Result<(), super::TryLockError> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.try_lock()
        }).await
    }

    pub async fn try_lock_shared(&self) -> Result<(), super::TryLockError> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.try_lock_shared()
        }).await
    }

    pub async fn unlock(&self) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.unlock()
        }).await
    }

    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.set_len(size)
        }).await
    }

    pub async fn metadata(&self) -> io::Result<super::Metadata> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.metadata()
        }).await
    }

    pub fn try_clone(&self) -> io::Result<File> {
        Ok(Self{
            io: Arc::new((*self.io).try_clone()?)
        })
    }

    pub async fn set_permissions(&self, perm: super::Permissions) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.set_permissions(perm.clone())
        }).await
    }

    pub async fn set_times(&self, time: super::FileTimes) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.set_times(time)
        }).await
    }

    pub async fn set_modified(&self, time: std::time::SystemTime) -> io::Result<()> {
        let inner = self.io.clone();
        thread_await( move||{
            inner.set_modified(time)
        }).await
    }
}

