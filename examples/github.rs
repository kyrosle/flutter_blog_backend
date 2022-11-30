use std::path::{Path, PathBuf};

use git2::Repository;

#[derive(Debug, Clone)]
/// warp a element [`PathBuf`] then for file path settings management.
pub struct SPath(PathBuf);

impl SPath {
    /// init path is the exe program directory, then you can set the follow path with a `&str` storage_path
    pub fn new_with_storage_path(storage_path: &str) -> SPath {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.push(storage_path);
        SPath(path)
    }
    /// push a path which can be a `&str` or other things impl the `AsRef<Path>`
    pub fn push(mut self, path: impl AsRef<Path>) -> Self {
        self.inner_mut().push(path);
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
    /// trans to path str
    pub fn to_str(&self) -> Result<&str, std::io::Error> {
        Ok(self.inner_ref().to_str().unwrap())
    }
    /// check this directory wether is exists, if exists return true,
    /// else the condition of path not exists or not a directory will return false.
    pub fn check_dir_exists(&self) -> bool {
        let path = self.to_str().unwrap();
        match std::fs::metadata(path) {
            Ok(meta) => matches!(meta.is_dir(), true),
            Err(_) => false,
        }
    }
    /// create the directory if not exists.
    pub fn create_dir_all(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self)?;
        Ok(())
    }
    /// remove the directory, if not exists nothing will happen.
    pub fn check_dir_exists_then_clean(&self) -> std::io::Result<()> {
        let path = self.to_str().unwrap();
        if self.check_dir_exists() {
            std::fs::remove_dir(path)?;
        }
        Ok(())
    }
}

impl AsRef<Path> for SPath {
    fn as_ref(&self) -> &Path {
        self.inner_ref().as_path()
    }
}

fn main() {
    let url = "https://github.com/kyrosle/salvo_t.git";
    let storage_path = SPath::new_with_storage_path("github_download").push("test");

    println!("{:#?}", storage_path);
    storage_path.create_dir_all().unwrap();
    assert!(storage_path.check_dir_exists())

    // let _ = match Repository::clone(url, storage_path) {
    //     Ok(repo) => repo,
    //     Err(e) => panic!("{}", e),
    // };
}
