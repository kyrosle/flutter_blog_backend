use anyhow::{anyhow, Result};
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::core::git_book_manager::config::read_book_config::BookConfigs;
use crate::core::{git_book_manager::config::ReadConfig, SPath};

pub static BOOKS_MANAGER: Lazy<Mutex<BookManager>> = Lazy::new(|| {
    Mutex::new(BookManager::new(
        BookConfigs::config_read_from_yaml_path("test-config/books-config.yml").unwrap(),
        None,
    ))
});

pub struct BookManager {
    configs: BookConfigs,
    storage_path: Option<SPath>,
}

impl BookManager {
    pub fn new(configs: BookConfigs, storage_path: Option<SPath>) -> Self {
        Self {
            configs,
            storage_path,
        }
    }
    pub fn iter_run(&self) -> Result<(), git2::Error> {
        // TODO: If the storage_path is None, we should set the default path is the executable application local directory.
        self.configs.run(self.storage_path.clone().unwrap())
    }
    pub fn hook_repository(&self, storage_path: SPath) -> Result<()> {
        self.configs.hook_repository(storage_path)
    }
    pub fn set_storage_path(&mut self, path: SPath) {
        self.storage_path = Some(path);
    }
}

impl ReadConfig for BookConfigs {
    fn config_read_from_yaml_path(path: &str) -> Result<Self> {
        let dir_path = SPath::new(path);

        if !dir_path.check_dir_exists() && !dir_path.check_dir_file_exists() {
            return Err(anyhow!(
                "Config Path is not exist {}",
                dir_path.to_str().unwrap()
            ));
        }
        Ok(Self::deserialize_config(dir_path).unwrap())
    }

    fn config_read_from_yaml_exec_path() -> Result<Self> {
        let dir_path = SPath::default().push("books-config.yml");

        if !dir_path.check_dir_exists() && !dir_path.check_dir_file_exists() {
            return Err(anyhow!(
                "Config Path is not exist {}",
                dir_path.to_str().unwrap()
            ));
        }
        Ok(Self::deserialize_config(dir_path).unwrap())
    }
}
