mod file;
mod path;

pub use file::{File, FileOptions, FileStat, SEEK_CUR, SEEK_END, SEEK_SET};
pub use path::{AsPath, DirEntry, Path};

use crate::PLAYDATE;
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_void;
use no_std_io::io::{self, Read, Write};

fn handle() -> &'static sys::playdate_file {
    unsafe { &*(*PLAYDATE.raw_api).file }
}

fn get_error() -> Option<io::Error> {
    let c_string = unsafe { handle().geterr.unwrap()() };
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

/// Copies the contents of one file to another. This function will also copy the permission bits of the original file to the destination file.
///
/// This function will overwrite the contents of to.
pub fn copy(from: impl AsPath, to: impl AsPath) -> io::Result<()> {
    let data = read(from)?;
    let mut f = File::open(to, FileOptions::Write)?;
    f.write(&data)?;
    Ok(())
}

/// Creates a new, empty directory at the provided path
pub fn create_dir(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { handle().mkdir.unwrap()(c_string.as_ptr()) };
    if result != 0 {
        Ok(())
    } else {
        Err(get_error().unwrap())
    }
}

// /// Creates a new directory at the provided path, and all parent directories as needed.
// pub fn create_dir_all(path: impl AsPath) -> io::Result<()> {
//     unimplemented!()
// }

/// Read file system metadata.
pub fn stat(path: impl AsPath) -> Result<FileStat, io::Error> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let mut stat = FileStat::default();
    let result = unsafe { handle().stat.unwrap()(c_string.as_ptr(), &mut stat) };
    if result == 0 {
        Ok(stat)
    } else {
        Err(get_error().unwrap())
    }
}

/// Returns true if the path points at an existing entity.
pub fn exists(path: impl AsPath) -> bool {
    stat(path).is_ok()
}

/// Reads the entire contents of a file into a bytes vector.
pub fn read(path: impl AsPath) -> io::Result<Vec<u8>> {
    let mut f = File::open(path, FileOptions::Read)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

/// List the contents of the directory.
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
        handle().listfiles.unwrap()(
            c_string.as_ptr(),
            Some(callback_wrapper),
            callback_dyn_ptr as *mut _,
            1 as i32,
        )
    };
    if result == 0 {
        Ok(files)
    } else {
        Err(get_error().unwrap())
    }
}

/// Removes an empty directory.
pub fn remove_dir(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { handle().unlink.unwrap()(c_string.as_ptr(), 0) };
    if result == 0 {
        Ok(())
    } else {
        Err(get_error().unwrap())
    }
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
pub fn remove_dir_all(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { handle().unlink.unwrap()(c_string.as_ptr(), 1) };
    if result == 0 {
        Ok(())
    } else {
        Err(get_error().unwrap())
    }
}

/// Removes a file from the filesystem.
pub fn remove_file(path: impl AsPath) -> io::Result<()> {
    let path = path.as_str();
    let c_string = CString::new(path.as_ref()).unwrap();
    let result = unsafe { handle().unlink.unwrap()(c_string.as_ptr(), 0) };
    if result == 0 {
        Ok(())
    } else {
        Err(get_error().unwrap())
    }
}

/// Renames a file or directory to a new name, replacing the original file if to already exists.
pub fn rename(from: impl AsPath, to: impl AsPath) -> io::Result<()> {
    let from = from.as_str();
    let to = to.as_str();
    let from_c_string = CString::new(from.as_ref()).unwrap();
    let to_c_string = CString::new(to.as_ref()).unwrap();
    let result = unsafe { handle().rename.unwrap()(from_c_string.as_ptr(), to_c_string.as_ptr()) };
    if result == 0 {
        Ok(())
    } else {
        Err(get_error().unwrap())
    }
}

/// Writes a slice as the entire contents of a file.
///
/// This function will create a file if it does not exist, and will entirely replace its contents if it does.
pub fn write(path: impl AsPath, data: impl AsRef<[u8]>) -> io::Result<()> {
    let mut f = File::open(path, FileOptions::Write)?;
    f.write(data.as_ref())?;
    Ok(())
}
