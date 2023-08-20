use alloc::string::String;

#[derive(Debug)]
pub enum Error {
    // Graphics
    FailedToLoadBitMapTableFromFile(String),
    FailedToLoadFont(String),
    FailedToSetBitmapMask,
    FailedToLoadBitMapFromFile(String),
    FailedToLoadBitMapFromBitMapTable(String),
    // File System
    FS(String),
    // Lua
    Lua(String),
    // All other unknown errors
    Unknown(String),
}
