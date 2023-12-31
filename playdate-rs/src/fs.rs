use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;

pub use sys::{FileOptions, FileStat, SEEK_CUR, SEEK_END, SEEK_SET};

use no_std_io::io::{self};

pub use no_std_io::io::{Read, Seek, Write};

pub struct PlaydateFileSystem {
    handle: *const sys::playdate_file,
}

impl PlaydateFileSystem {
    pub(crate) fn new(handle: *const sys::playdate_file) -> Self {
        Self { handle }
    }

    /// Returns human-readable text describing the most recent error (usually indicated by a -1 return from a filesystem function).
    pub fn get_error(&self) -> Option<io::Error> {
        let c_string = unsafe { (*self.handle).geterr.unwrap()() };
        if c_string.is_null() {
            None
        } else {
            let c_str = unsafe { ::core::ffi::CStr::from_ptr(c_string) };
            Some(io::Error::new(
                io::ErrorKind::Other,
                c_str.to_str().unwrap(),
            ))
        }
    }

    /// Calls the given callback function for every file at path. Subfolders are indicated by a trailing slash '/' in filename. listfiles() does not recurse into subfolders. If showhidden is set, files beginning with a period will be included; otherwise, they are skipped. Returns 0 on success, -1 if no folder exists at path or it can’t be opened.
    pub fn list_files(
        &self,
        path: impl AsRef<str>,
        show_hidden: bool,
        mut callback: impl FnMut(&str),
    ) -> Result<(), io::Error> {
        let c_string = CString::new(path.as_ref()).unwrap();
        extern "C" fn callback_wrapper(filename: *const i8, callback: *mut c_void) {
            let callback = callback as *mut *mut dyn FnMut(&str);
            let callback = unsafe { &mut **callback };
            let filename = unsafe { ::core::ffi::CStr::from_ptr(filename) };
            callback(filename.to_str().unwrap());
        }
        let mut callback_dyn: *mut dyn FnMut(&str) = &mut callback;
        let callback_dyn_ptr: *mut *mut dyn FnMut(&str) = &mut callback_dyn;
        let result = unsafe {
            (*self.handle).listfiles.unwrap()(
                c_string.as_ptr(),
                Some(callback_wrapper),
                callback_dyn_ptr as *mut _,
                show_hidden as i32,
            )
        };
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Populates the FileStat stat with information about the file at path. Returns 0 on success, or -1 in case of error.
    pub fn stat(&self, path: impl AsRef<str>) -> io::Result<FileStat> {
        let c_string = CString::new(path.as_ref()).unwrap();
        let mut stat = FileStat::default();
        let result = unsafe { (*self.handle).stat.unwrap()(c_string.as_ptr(), &mut stat) };
        if result != 0 {
            Ok(stat)
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Creates the given path in the Data/&lt;gameid&gt; folder. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.
    pub fn mkdir(&self, path: impl AsRef<str>) -> io::Result<()> {
        let c_string = CString::new(path.as_ref()).unwrap();
        let result = unsafe { (*self.handle).mkdir.unwrap()(c_string.as_ptr()) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Deletes the file at path. Returns 0 on success, or -1 in case of error. If recursive is 1 and the target path is a folder, this deletes everything inside the folder (including folders, folders inside those, and so on) as well as the folder itself.
    pub fn unlink(&self, name: impl AsRef<str>, recursive: bool) -> io::Result<()> {
        let c_string = CString::new(name.as_ref()).unwrap();
        let result = unsafe { (*self.handle).unlink.unwrap()(c_string.as_ptr(), recursive as i32) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Renames the file at from to to. It will overwrite the file at to without confirmation. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.
    pub fn rename(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> io::Result<()> {
        let from_c_string = CString::new(from.as_ref()).unwrap();
        let to_c_string = CString::new(to.as_ref()).unwrap();
        let result =
            unsafe { (*self.handle).rename.unwrap()(from_c_string.as_ptr(), to_c_string.as_ptr()) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Opens a handle for the file at path. The kFileRead mode opens a file in the game pdx, while kFileReadData searches the game’s data folder; to search the data folder first then fall back on the game pdx, use the bitwise combination kFileRead|kFileReadData.kFileWrite and kFileAppend always write to the data folder. The function returns NULL if a file at path cannot be opened, and playdate->file->geterr() will describe the error. The filesystem has a limit of 64 simultaneous open files.
    pub fn open(&self, name: impl AsRef<str>, mode: FileOptions) -> io::Result<File> {
        let c_string = CString::new(name.as_ref()).unwrap();
        let file = unsafe { (*self.handle).open.unwrap()(c_string.as_ptr(), mode) };
        if file.is_null() {
            Err(self.get_error().unwrap())
        } else {
            Ok(File::new(file))
        }
    }

    /// Closes the given file handle. Returns 0 on success, or -1 in case of error.
    pub(crate) fn close(&self, file: *mut sys::SDFile) -> io::Result<()> {
        let result = unsafe { (*self.handle).close.unwrap()(file) };
        if result == 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Reads up to len bytes from the file into the buffer buf. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.
    pub(crate) fn read(&self, file: *mut sys::SDFile, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe {
            (*self.handle).read.unwrap()(file, buf.as_mut_ptr() as *mut _, buf.len() as u32)
        };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Writes the buffer of bytes buf to the file. Returns the number of bytes written, or -1 in case of error.
    pub(crate) fn write(&self, file: *mut sys::SDFile, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            (*self.handle).write.unwrap()(file, buf.as_ptr() as *const _, buf.len() as u32)
        };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Flushes the output buffer of file immediately. Returns the number of bytes written, or -1 in case of error.
    pub(crate) fn flush(&self, file: *mut sys::SDFile) -> io::Result<()> {
        let result = unsafe { (*self.handle).flush.unwrap()(file) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Returns the current read/write offset in the given file handle, or -1 on error.
    pub(crate) fn tell(&self, file: *mut sys::SDFile) -> io::Result<usize> {
        let result = unsafe { (*self.handle).tell.unwrap()(file) };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Sets the read/write offset in the given file handle to pos, relative to the whence macro. SEEK_SET is relative to the beginning of the file, SEEK_CUR is relative to the current position of the file pointer, and SEEK_END is relative to the end of the file. Returns 0 on success, -1 on error.
    pub(crate) fn seek(&self, file: *mut sys::SDFile, pos: usize, whence: i32) -> io::Result<()> {
        let result = unsafe { (*self.handle).seek.unwrap()(file, pos as i32, whence) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }
}

use core::ffi::c_void;

use crate::PLAYDATE;

pub struct File {
    handle: *mut sys::SDFile,
}

impl File {
    pub(crate) fn new(handle: *mut sys::SDFile) -> Self {
        Self { handle }
    }

    /// Returns the current read/write offset in the given file handle, or -1 on error.
    pub fn tell(&self) -> io::Result<usize> {
        PLAYDATE.file.tell(self.handle)
    }

    /// Open a new file
    pub fn open(name: impl AsRef<str>, mode: FileOptions) -> io::Result<Self> {
        PLAYDATE.file.open(name, mode)
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
        let Ok(size) = PLAYDATE.file.read(self.handle, buf) else {
            return Err(io::Error::new(io::ErrorKind::Other, "file read error"));
        };
        Ok(size)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let Ok(size) = PLAYDATE.file.write(self.handle, buf) else {
            return Err(io::Error::new(io::ErrorKind::Other, "file write error"));
        };
        Ok(size)
    }

    fn flush(&mut self) -> io::Result<()> {
        if PLAYDATE.file.flush(self.handle).is_err() {
            Err(io::Error::new(io::ErrorKind::Other, "file flush error"))
        } else {
            Ok(())
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
        if PLAYDATE.file.seek(self.handle, pos, whence as _).is_err() {
            Err(io::Error::new(io::ErrorKind::Other, "file seek error"))
        } else {
            Ok(pos as u64)
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        PLAYDATE.file.close(self.handle).unwrap();
    }
}
