#![allow(unused_imports)]
pub use std::io::{Error, ErrorKind, IoSlice, IoSliceMut, Result, SeekFrom};

mod read;
pub use read::*;
mod write;
pub use write::*;
mod seek;
pub use seek::*;
