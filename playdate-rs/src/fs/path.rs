use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Div;

pub use no_std_io::io::{self, Write};

#[derive(PartialEq, Eq, Clone)]
pub struct Path {
    segments: Vec<String>,
    is_absolute: bool,
}

impl Path {
    /// Create a new Path from a string.
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

    /// Return true if the path is absolute.
    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    /// Get the components of the path.
    pub fn components(&self) -> &[String] {
        &self.segments
    }

    /// Get parent directory of the path.
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

    /// Concatenate this path with another component.
    pub fn join(&self, other: impl AsRef<str>) -> Self {
        let mut new_segments = self.segments.clone();
        new_segments.push(other.as_ref().to_string());
        Self {
            segments: new_segments,
            is_absolute: self.is_absolute,
        }
    }

    /// Check if the path exists.
    pub fn exists(&self) -> bool {
        super::exists(self)
    }

    /// Check if the path is a file.
    pub fn is_file(&self) -> bool {
        super::stat(self).map_or(false, |stat| stat.isdir == 0)
    }

    /// Check if the path is a directory.
    pub fn is_dir(&self) -> bool {
        super::stat(self).map_or(false, |stat| stat.isdir != 0)
    }

    /// Extracts the extension (without the leading dot) of self.file_name, if possible.
    pub fn extension(&self) -> Option<&str> {
        if let Some(last_segment) = self.segments.last() {
            if let Some(pos) = last_segment.rfind('.') {
                return Some(&last_segment[pos + 1..]);
            }
        }
        None
    }

    /// Returns the final component of the Path, if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it’s the path of a directory, this is the directory name.
    pub fn file_name(&self) -> Option<&str> {
        if let Some(last_segment) = self.segments.last() {
            return Some(last_segment);
        }
        None
    }

    /// Extracts the stem (non-extension) portion of self.file_name.
    pub fn file_stem(&self) -> Option<&str> {
        if let Some(last_segment) = self.segments.last() {
            if let Some(pos) = last_segment.rfind('.') {
                return Some(&last_segment[..pos]);
            }
        }
        None
    }

    /// List the contents of the directory.
    pub fn read_dir(&self) -> io::Result<Vec<Path>> {
        let self_rc = alloc::rc::Rc::new(self.clone());
        let names = super::read_dir(self)?;
        let mut entries = Vec::new();
        for name in names {
            let entry = DirEntry::new(self_rc.clone(), name);
            entries.push(entry.path());
        }
        Ok(entries)
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
