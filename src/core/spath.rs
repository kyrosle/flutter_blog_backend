#![allow(dead_code)]
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
/// warp a element [`PathBuf`] then for file path settings management.
/// and the path default is from the executable application directory or accept self define path
pub struct SPath(PathBuf);

impl SPath {
    /// new with a self configured path `&str`
    pub fn new(path: &str) -> SPath {
        let path = PathBuf::from(path);
        SPath(path)
    }
    /// init path is the exe program directory, then you can set the follow path with a `&str` storage_path
    pub fn new_with_storage_path(storage_path: &str) -> SPath {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.push(storage_path);
        SPath(path)
    }
    /// set the follow path with a `&str` storage_path
    pub fn set_with_storage_path(mut self, storage_path: &str) -> Self {
        self.inner_mut().push(storage_path);
        self
    }
    /// get reference
    pub fn inner_ref(&self) -> &PathBuf {
        &self.0
    }
    /// get mutable reference
    pub fn inner_mut(&mut self) -> &mut PathBuf {
        &mut self.0
    }
    /// get the ownership
    pub fn inner(self) -> PathBuf {
        self.0
    }
    /// Yields a [&str] slice if the Path is valid unicode.
    /// This conversion may entail doing a check for UTF-8 validity.
    /// Note that validation is performed because non-UTF-8 strings are perfectly valid for some OS.
    pub fn to_str(&self) -> Option<&str> {
        self.inner_ref().to_str()
    }
    /// Extends self with path.
    ///
    /// If path is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if path has a root but no prefix (e.g., \windows), it replaces everything except for the prefix (if any) of self.
    /// * if path has a prefix but no root, it replaces self.
    /// * if self has a verbatim prefix (e.g. \\?\C:\windows) and path is not empty, the new path is normalized: all references to . and .. are removed.
    pub fn push(mut self, path: impl AsRef<Path>) -> Self {
        self.inner_mut().push(path);
        self
    }
    /// Truncates self to self.0.parent.
    /// Returns false and does nothing if self.0.parent is None. Otherwise, returns true.
    pub fn pop_check_parent(&mut self) -> bool {
        self.inner_mut().pop()
    }
    // pop function of chain call
    // if self.0.parent is None, then nothing will happen
    pub fn pop(mut self) -> Self {
        let _ = self.pop_check_parent();
        self
    }
    /// Invokes clear on the underlying instance of OsString.
    pub fn clear(&mut self) {
        self.inner_mut().clear()
    }
    /// Check this directory wether is exists, if exists return true,
    /// else the condition of path not exists or not a directory will return false.
    pub fn check_dir_exists(&self) -> bool {
        let path = self.to_str().unwrap();
        match std::fs::metadata(path) {
            Ok(meta) => matches!(meta.is_dir(), true),
            Err(_) => false,
        }
    }
    /// Checking in this directory, the file wether is exists, if exists return true,
    /// else the condition of path not exists or not a directory will return false.
    pub fn check_dir_file_exists(&self) -> bool {
        let file_path = self.to_str().unwrap();
        match std::fs::metadata(file_path) {
            Ok(meta) => matches!(meta.is_file(), true),
            Err(_) => false,
        }
    }
    /// Updates self.0.extension to extension.
    /// Does nothing if self.0.file_name is None, updates the extension otherwise.
    /// If self.0.extension is None, the extension is added; otherwise it is replaced.
    pub fn set_extension<S: AsRef<std::ffi::OsStr>>(mut self, extension: S) -> Self {
        self.inner_mut().set_extension(extension);
        self
    }
    /// Updates self.0.file_name to file_name.
    /// If self.0.file_name was None, this is equivalent to pushing file_name.
    /// Otherwise it is equivalent to calling pop and then pushing file_name.
    /// The new path will be a sibling of the original path.
    /// (That is, it will have the same parent.)
    pub fn set_file_name<S: AsRef<std::ffi::OsStr>>(mut self, file_name: S) -> Self {
        self.inner_mut().set_file_name(file_name);
        self
    }
    /// create the directory if not exists.
    ///
    /// Considering the path if is a file path that the file name will have a extension name, like `test.txt`.
    pub fn create_dir_all(&self) -> std::io::Result<()> {
        if self.inner_ref().extension().is_some() {
            let parent = self.inner_ref().parent().unwrap();
            std::fs::create_dir_all(parent)?;
            std::fs::File::create(self)?;
        } else {
            std::fs::create_dir_all(self)?;
        }
        Ok(())
    }
    /// remove the directory, if not exists nothing will happen.
    pub fn check_dir_exists_then_clean(&self) -> std::io::Result<()> {
        let path = self.to_str().unwrap();
        if self.check_dir_exists() || self.check_dir_file_exists() {
            if self.inner_ref().extension().is_some() {
                std::fs::remove_file(self)?;
            } else {
                std::fs::remove_dir(path)?;
            }
        }
        Ok(())
    }
}

fn check_is_file(path: impl AsRef<Path>) -> bool {
    std::fs::File::open(path).is_ok()
}

impl Default for SPath {
    /// default is from the executable application directory
    fn default() -> SPath {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        SPath(path)
    }
}

impl AsRef<Path> for SPath {
    fn as_ref(&self) -> &Path {
        self.inner_ref().as_path()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn test_spath_to_str() {
        assert_eq!(SPath::new(".").to_str().unwrap(), ".");
        assert_eq!(SPath::new("./test").to_str().unwrap(), "./test");
        assert_eq!(SPath::new("./test/tt").to_str().unwrap(), "./test/tt");
    }
    #[test]
    #[cfg(windows)]
    fn test_setting_function() {
        let dir = SPath::new(".");
        let root_dir = ".".to_string();
        assert_eq!(dir.to_str().unwrap(), root_dir, "new assert_eq");

        let dir = dir.push("test");
        let temp_right_dir = format!("{}\\test", root_dir);
        assert_eq!(dir.to_str().unwrap(), temp_right_dir, "push assert_eq");

        let dir = dir.pop();
        assert_eq!(dir.to_str().unwrap(), root_dir, "pop assert_eq");

        let dir = dir.push("test_file.txt");
        assert_eq!(
            dir.to_str().unwrap(),
            format!("{}\\test_file.txt", root_dir),
            "push file assert_eq"
        );
        let dir = dir.pop();

        let dir = dir.push("test").push("test_file.txt");
        assert_eq!(
            dir.to_str().unwrap(),
            format!("{}\\test\\test_file.txt", root_dir),
            "push path->push file assert_eq"
        );
    }
    #[test]
    fn test_check_dir() {
        let temp_test_dir = tempdir().unwrap();
        let temp_test_dir = temp_test_dir.path().to_str().unwrap();

        let path = SPath::new(temp_test_dir).push("check_dir");
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir should not exist, and not a file"
        );
        path.create_dir_all().unwrap();
        assert!(
            path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir should exist, and not a file"
        );
        let path = path.push("level1").push("level2");
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir/level1/level2 should not exist, and not a file"
        );
        path.create_dir_all().unwrap();
        assert!(
            path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir/level1/level2 should exist, and not a file"
        );

        path.check_dir_exists_then_clean().unwrap();
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir/level1/level2 should not exist, and not a file"
        );
    }

    #[test]
    fn test_check_file() {
        let temp_dir = tempdir().unwrap();
        let temp_test_dir = temp_dir.path().to_str().unwrap();

        let path = SPath::new(temp_test_dir).push("check_file.txt");
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_file.txt should not exist, and not a file"
        );
        path.create_dir_all().unwrap();
        assert!(
            !path.check_dir_exists() && path.check_dir_file_exists(),
            "tempdir, the /check_file.txt should not exist, and is a file"
        );
        path.check_dir_exists_then_clean().unwrap();
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_file.txt should not exist, and not a file"
        );
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_check_dir_file() {
        let temp_dir = tempdir().unwrap();
        let temp_test_dir = temp_dir.path().to_str().unwrap();

        let path = SPath::new(temp_test_dir).push("check_dir");
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir should not exist, and not a file"
        );
        let path = path.push("test.txt");
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir/test.txt should not exist, and not a file"
        );
        path.create_dir_all().unwrap();
        assert!(
            !path.check_dir_exists() && path.check_dir_file_exists(),
            "tempdir, the /check_dir/test.txt should not exist, and is a file"
        );
        path.check_dir_exists_then_clean().unwrap();
        assert!(
            !path.check_dir_exists() && !path.check_dir_file_exists(),
            "tempdir, the /check_dir/test.txt should not exist, and not a file"
        );
        temp_dir.close().unwrap();
    }
}
