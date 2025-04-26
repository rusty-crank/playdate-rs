use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;

pub use no_std_io::io::{self, Read, Seek, Write};
pub use sys::{FileOptions, FileStat, SEEK_CUR, SEEK_END, SEEK_SET};

use super::{get_error, handle, AsPath};

pub struct File {
    pub(crate) handle: *mut sys::SDFile,
}

impl File {
    pub(crate) fn new(handle: *mut sys::SDFile) -> Self {
        Self { handle }
    }

    /// Returns the current read/write offset in the given file handle, or -1 on error.
    pub fn tell(&self) -> io::Result<usize> {
        let result = unsafe { handle().tell.unwrap()(self.handle) };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(get_error().unwrap())
        }
    }

    /// Open a new file
    pub fn open(name: impl AsPath, mode: FileOptions) -> io::Result<Self> {
        let name = name.as_str();
        let c_string = CString::new(name.as_ref()).unwrap();
        let file = unsafe { handle().open.unwrap()(c_string.as_ptr(), mode) };
        if file.is_null() {
            Err(get_error().unwrap())
        } else {
            Ok(File::new(file))
        }
    }

    /// Read the entire content to a string
    pub fn read_to_string(&mut self) -> io::Result<String> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        Ok(String::from_utf8(buf).unwrap())
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe {
            handle().read.unwrap()(self.handle, buf.as_mut_ptr() as *mut _, buf.len() as u32)
        };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(get_error().unwrap())
        }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            handle().write.unwrap()(self.handle, buf.as_ptr() as *const _, buf.len() as u32)
        };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(get_error().unwrap())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = unsafe { handle().flush.unwrap()(self.handle) };
        if result == 0 {
            Ok(())
        } else {
            Err(get_error().unwrap())
        }
    }
}

impl Seek for File {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let whence = match pos {
            io::SeekFrom::Start(_) => SEEK_SET,
            io::SeekFrom::End(_) => SEEK_END,
            io::SeekFrom::Current(_) => SEEK_CUR,
        };
        let pos = match pos {
            io::SeekFrom::Start(pos) => pos as usize,
            io::SeekFrom::End(pos) => pos as usize,
            io::SeekFrom::Current(pos) => pos as usize,
        };
        let result = unsafe { handle().seek.unwrap()(self.handle, pos as i32, whence as _) };
        if result == 0 {
            Ok(pos as _)
        } else {
            Err(get_error().unwrap())
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let result = unsafe { handle().close.unwrap()(self.handle) };
        assert!(result == 0, "Failed to close file");
    }
}
