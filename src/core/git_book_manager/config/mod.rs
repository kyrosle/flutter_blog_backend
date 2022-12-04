use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub mod read_book_config;
pub mod read_server_config;


pub trait ReadConfig
where
    Self: Serialize + DeserializeOwned + Sized,
{
    /// Read the config yml from the executable application directory
    fn config_read_from_yaml_exec_path() -> Result<Self>;
    /// Read the config yml from the given path
    fn config_read_from_yaml_path(path: &str) -> Result<Self>;

    fn deserialize_config(dir_path: impl AsRef<std::path::Path>) -> Result<Self> {
        // read the file context
        let yml = String::from_utf8(std::fs::read(dir_path).unwrap()).unwrap();
        let configs: Self = serde_yaml::from_str(&yml).unwrap();
        Ok(configs)
    }
}
