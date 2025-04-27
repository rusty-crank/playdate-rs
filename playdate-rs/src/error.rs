pub use crate::sys::PDNetErr as NetworkError;
use alloc::string::String;
use no_std_io::io;

#[derive(Debug)]
pub enum Error {
    // Graphics
    FailedToLoadBitMapTableFromFile(String),
    FailedToLoadFont(String),
    FailedToSetBitmapMask,
    FailedToLoadBitMapFromFile(String),
    FailedToLoadBitMapFromBitMapTable(String),
    // IO Error
    IO(io::Error),
    FileNotExists(String),
    NetworkError(NetworkError),
    PermissionDenied,
    // Lua
    Lua(String),
    // All other unknown errors
    Unknown(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<io::ErrorKind> for Error {
    fn from(err: io::ErrorKind) -> Self {
        Error::IO(err.into())
    }
}
