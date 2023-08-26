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
    // Lua
    Lua(String),
    // All other unknown errors
    Unknown(String),
}
