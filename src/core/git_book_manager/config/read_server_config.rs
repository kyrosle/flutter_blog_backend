use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::core::SPath;

use super::ReadConfig;

/// basic server node config
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub domains: String,
    pub ip: String,
    pub port: usize,
}

/// load from configs/server-config.yml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfigs {
    #[serde(flatten)]
    pub configs: HashMap<String, ServerConfig>,
}

impl ReadConfig for ServerConfigs {
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
        let dir_path = SPath::default().push("server-config.yml");

        if !dir_path.check_dir_exists() && !dir_path.check_dir_file_exists() {
            return Err(anyhow!(
                "Config Path is not exist {}",
                dir_path.to_str().unwrap()
            ));
        }
        Ok(Self::deserialize_config(dir_path).unwrap())
    }
}
