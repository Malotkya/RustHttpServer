pub use std::fs::{
    canonicalize,
    Metadata, Permissions, FileTimes, FileType, DirEntry,
    TryLockError
};
use std::{
    path::Path,
    io
};
use crate::thread_await;

macro_rules! asyncify {
    ($name:ident($( $( $arg_name:ident: $arg_type:ident ),+ )? ) -> $output:ty) => {
        pub async fn $name $(< $($arg_type: AsRef<Path>),+ >)?
        (
            $( $( $arg_name: $arg_type ),+ )?
        ) -> $output {
            $(
                $(let $arg_name = canonicalize($arg_name)?; )+
            )?

            thread_await(move ||{
                std::fs::$name(
                    $( $( $arg_name.clone() ),+ )?
                )
            }).await
        }
    };
}

mod dir_builder;
pub use dir_builder::DirBuilder;
mod file;
pub use file::File;
mod open_options;
pub use open_options::OpenOptions;
mod read_dir;
pub use read_dir::ReadDir;

asyncify!(copy(from: P, to: Q) -> io::Result<u64>);
asyncify!(create_dir(path: P) -> io::Result<()>);
asyncify!(create_dir_all(path: P) -> io::Result<()>);
asyncify!(exists(path: P) -> io::Result<bool>);
asyncify!(hard_link(original: P, link: Q) -> io::Result<()>);
asyncify!(metadata(path: P) -> io::Result<Metadata>);
asyncify!(read(path: P) -> io::Result<Vec<u8>>);
asyncify!(read_link(path: P) -> io::Result<std::path::PathBuf>);
asyncify!(read_to_string(path: P) -> io::Result<String>);
asyncify!(remove_dir(path: P) -> io::Result<()>);
asyncify!(remove_dir_all(path: P) -> io::Result<()>);
asyncify!(remove_file(path: P) -> io::Result<()>);
asyncify!(rename(from: P, to: Q) -> io::Result<()>);
// Depretiated: asyncify!(soft_link(original: P, link: Q) -> io::Result<()>);
asyncify!(symlink_metadata(path: P) -> io::Result<Metadata>);

pub async fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> io::Result<()> {
    let path = canonicalize(path)?;

    thread_await(move||{
        std::fs::set_permissions(path.clone(), perm.clone())
    }).await
}

pub async fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    let path = canonicalize(path)?;
    let ptr = contents.as_ref().as_ptr() as usize;
    let len = contents.as_ref().len();

    thread_await(move ||{
        let contents = unsafe {
            std::slice::from_raw_parts(ptr as *const u8, len)
        };
        std::fs::write(
            path.clone(),
            contents
        )
    }).await
}