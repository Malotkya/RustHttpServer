pub mod fs;
pub mod io;
pub mod net;
mod promise;
pub use promise::Promise;

pub(crate) fn clone_io_result<T: Clone>(result: &io::Result<T>) -> io::Result<T> {
    match result {
        Ok(t) => Ok(t.clone()),
        Err(e) => Err(
            std::io::Error::new(
                e.kind(),
                e.to_string()
            )
        )
    }
}