use anyhow::{anyhow, Result};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::SPath;

use super::ReadConfig;

/// Book node config data structure.
///
/// `path` is about the git-url from the web.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BookConfig {
    pub path: String,
}

/// A set of [`BookConfig`], which loading from configure file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct BookConfigs {
    // allow the child - structure serde
    #[serde(flatten)]
    pub configs: HashMap<String, BookConfig>,
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
