use alloc::borrow::Cow;
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Div;

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
    fn get_error(&self) -> Option<io::Error> {
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

    /// Reads up to len bytes from the file into the buffer buf. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.
    fn read(&self, file: *mut sys::SDFile, buf: &mut [u8]) -> io::Result<usize> {
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
    fn write(&self, file: *mut sys::SDFile, buf: &[u8]) -> io::Result<usize> {
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
    fn flush(&self, file: *mut sys::SDFile) -> io::Result<()> {
        let result = unsafe { (*self.handle).flush.unwrap()(file) };
        if result == 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Returns the current read/write offset in the given file handle, or -1 on error.
    fn tell(&self, file: *mut sys::SDFile) -> io::Result<usize> {
        let result = unsafe { (*self.handle).tell.unwrap()(file) };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Sets the read/write offset in the given file handle to pos, relative to the whence macro. SEEK_SET is relative to the beginning of the file, SEEK_CUR is relative to the current position of the file pointer, and SEEK_END is relative to the end of the file. Returns 0 on success, -1 on error.
    fn seek(&self, file: *mut sys::SDFile, pos: usize, whence: i32) -> io::Result<()> {
        let result = unsafe { (*self.handle).seek.unwrap()(file, pos as i32, whence) };
        if result == 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }
}

use core::ffi::c_void;

use crate::PLAYDATE;

pub struct File {
    pub(crate) handle: *mut sys::SDFile,
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
    pub fn open(name: impl AsPath, mode: FileOptions) -> io::Result<Self> {
        let name = name.as_str();
        let c_string = CString::new(name.as_ref()).unwrap();
        let file = unsafe { (*PLAYDATE.file.handle).open.unwrap()(c_string.as_ptr(), mode) };
        if file.is_null() {
            Err(PLAYDATE.file.get_error().unwrap())
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
        let result = unsafe { (*PLAYDATE.file.handle).close.unwrap()(self.handle) };
        assert!(result == 0, "Failed to close file");
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Path {
    segments: Vec<String>,
    is_absolute: bool,
}

impl Path {
    pub fn new(path: impl AsRef<str>) -> Self {
        let path = path.as_ref();
        let is_absolute = path.starts_with('/');
        let segments = path
            .split('/')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Self {
            segments,
            is_absolute,
        }
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn components(&self) -> &[String] {
        &self.segments
    }

    pub fn join(&self, other: impl AsRef<str>) -> Self {
        let mut new_segments = self.segments.clone();
        new_segments.push(other.as_ref().to_string());
        Self {
            segments: new_segments,
            is_absolute: self.is_absolute,
        }
    }

    pub fn exists(&self) -> bool {
        exists(self)
    }

    pub fn parent(&self) -> Option<Self> {
        if self.segments.len() > 1 {
            let parent_segments = &self.segments[..self.segments.len() - 1];
            Some(Self {
                segments: parent_segments.to_vec(),
                is_absolute: self.is_absolute,
            })
        } else {
            None
        }
    }

    pub fn is_file(&self) -> bool {
        stat(self).map_or(false, |stat| stat.isdir == 0)
    }

    pub fn is_dir(&self) -> bool {
        stat(self).map_or(false, |stat| stat.isdir != 0)
    }

    fn as_cow_str(&self) -> Cow<str> {
        let mut path = String::new();
        for segment in &self.segments {
            path.push('/');
            path.push_str(segment);
        }
        if !self.is_absolute {
            path.remove(0);
        }
        Cow::Owned(path)
    }

    pub fn extension(&self) -> Option<&str> {
        if let Some(last_segment) = self.segments.last() {
            if let Some(pos) = last_segment.rfind('.') {
                return Some(&last_segment[pos + 1..]);
            }
        }
        None
    }

    pub fn file_name(&self) -> Option<&str> {
        if let Some(last_segment) = self.segments.last() {
            return Some(last_segment);
        }
        None
    }

    pub fn file_stem(&self) -> Option<&str> {
        if let Some(last_segment) = self.segments.last() {
            if let Some(pos) = last_segment.rfind('.') {
                return Some(&last_segment[..pos]);
            }
        }
        None
    }

    pub fn read_dir(&self) -> io::Result<Vec<Path>> {
        let self_rc = alloc::rc::Rc::new(self.clone());
        let names = read_dir(self)?;
        let mut entries = Vec::new();
        for name in names {
            let entry = DirEntry::new(self_rc.clone(), name);
            entries.push(entry.path());
        }
        Ok(entries)
    }
}

pub struct DirEntry {
    dir: alloc::rc::Rc<Path>,
    name: String,
}

impl DirEntry {
    fn new(dir: alloc::rc::Rc<Path>, name: String) -> Self {
        Self { dir, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> Path {
        self.dir.join(&self.name)
    }
}

pub trait AsPath {
    fn as_path(&self) -> Path;
    fn as_str(&self) -> Cow<str>;
}

impl AsPath for Path {
    fn as_path(&self) -> Path {
        self.clone()
    }
    fn as_str(&self) -> Cow<str> {
        self.as_cow_str()
    }
}

impl AsPath for &Path {
    fn as_path(&self) -> Path {
        (*self).clone()
    }
    fn as_str(&self) -> Cow<str> {
        self.as_cow_str()
    }
}

impl AsPath for &str {
    fn as_path(&self) -> Path {
        Path::new(self)
    }
    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl AsPath for str {
    fn as_path(&self) -> Path {
        Path::new(self)
    }
    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl AsPath for &String {
    fn as_path(&self) -> Path {
        Path::new(self)
    }
    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl AsPath for String {
    fn as_path(&self) -> Path {
        Path::new(self)
    }
    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl Div for Path {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let mut new_segments = self.segments.clone();
        new_segments.extend(other.segments);
        Self {
            segments: new_segments,
            is_absolute: self.is_absolute,
        }
    }
}

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = self.as_cow_str();
        let s = s.as_ref();
        write!(f, "{}", s)
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = self.as_cow_str();
        let s = s.as_ref();
        write!(f, "{}", s)
    }
}

pub fn copy(from: impl AsPath, to: impl AsPath) -> io::Result<()> {
    let data = read(from)?;
    let mut f = File::open(to, FileOptions::Write)?;
    f.write(&data)?;
    Ok(())
}

pub fn create_dir(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { (*PLAYDATE.file.handle).mkdir.unwrap()(c_string.as_ptr()) };
    if result != 0 {
        Ok(())
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

// pub fn create_dir_all(path: impl AsPath) -> io::Result<()> {
//     let c_string = CString::new(path.as_ref()).unwrap();
//     let result = unsafe { (*PLAYDATE.file.handle).mkdirall.unwrap()(c_string.as_ptr()) };
//     if result != 0 {
//         Ok(())
//     } else {
//         Err(PLAYDATE.file.get_error().unwrap())
//     }
// }

pub fn stat(path: impl AsPath) -> Result<FileStat, io::Error> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let mut stat = FileStat::default();
    let result = unsafe { (*PLAYDATE.file.handle).stat.unwrap()(c_string.as_ptr(), &mut stat) };
    if result == 0 {
        Ok(stat)
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

pub fn exists(path: impl AsPath) -> bool {
    stat(path).is_ok()
}

pub fn read(path: impl AsPath) -> io::Result<Vec<u8>> {
    let mut f = File::open(path, FileOptions::Read)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_dir(path: impl AsPath) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let path = path.as_str();
    let mut callback = |filename: &str| {
        files.push(filename.to_string());
    };
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
        (*PLAYDATE.file.handle).listfiles.unwrap()(
            c_string.as_ptr(),
            Some(callback_wrapper),
            callback_dyn_ptr as *mut _,
            1 as i32,
        )
    };
    if result == 0 {
        Ok(files)
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

/// Removes an empty directory.
pub fn remove_dir(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { (*PLAYDATE.file.handle).unlink.unwrap()(c_string.as_ptr(), 0) };
    if result == 0 {
        Ok(())
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
pub fn remove_dir_all(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { (*PLAYDATE.file.handle).unlink.unwrap()(c_string.as_ptr(), 1) };
    if result == 0 {
        Ok(())
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

pub fn remove_file(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { (*PLAYDATE.file.handle).unlink.unwrap()(c_string.as_ptr(), 0) };
    if result == 0 {
        Ok(())
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

pub fn rename(from: impl AsPath, to: impl AsPath) -> io::Result<()> {
    let from = from.as_str();
    let to = to.as_str();
    let from_c_string = CString::new(from.as_ref()).unwrap();
    let to_c_string = CString::new(to.as_ref()).unwrap();
    let result = unsafe {
        (*PLAYDATE.file.handle).rename.unwrap()(from_c_string.as_ptr(), to_c_string.as_ptr())
    };
    if result == 0 {
        Ok(())
    } else {
        Err(PLAYDATE.file.get_error().unwrap())
    }
}

pub fn write(path: impl AsPath, data: impl AsRef<[u8]>) -> io::Result<()> {
    let mut f = File::open(path, FileOptions::Write)?;
    f.write(data.as_ref())?;
    Ok(())
}
