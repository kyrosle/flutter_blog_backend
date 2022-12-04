use anyhow::{anyhow, Result};
use git2::Repository;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{core::SPath, utils::git_pull::run};

/// Book node config data structure.
///
/// `path` is about the git-url from the web.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BookConfig {
    pub remote: Option<String>,
    pub branch: Option<String>,
}

impl BookConfig {
    /// If remote is None default is `origin`.
    ///
    /// If branch is None default is `master`.
    pub fn new(remote: Option<String>, branch: Option<String>) -> Self {
        Self { remote, branch }
    }
    pub fn run(&self, path: SPath) -> Result<(), git2::Error> {
        run(self.remote.clone(), self.branch.clone(), path)
    }
    pub fn get_remote(&self) -> Option<String> {
        self.remote.clone()
    }
    pub fn get_branch(&self) -> Option<String> {
        self.branch.clone()
    }
}

/// A set of [`BookConfig`], which loading from configure file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct BookConfigs {
    // allow the child - structure serde
    #[serde(flatten)]
    pub configs: HashMap<String, BookConfig>,
}
impl BookConfigs {
    pub fn run(&self, storage_path: SPath) -> Result<(), git2::Error> {
        for (name, config) in self.configs.iter() {
            println!(
                "git pull - name: {} - remote: {:?} - branch : {:?}",
                name, config.remote, config.branch
            );
            let storage_path = storage_path.clone();
            let _ = config.run(storage_path.push(name));
        }
        Ok(())
    }
    pub fn hook_repository(&self, storage_path: SPath) -> Result<()> {
        for (node_name, book_config) in self.configs.iter() {
            let work_dir = storage_path.clone().push(node_name);

            match Repository::open(work_dir.clone()) {
                // try to open the repository in the local directory
                Ok(_) => {
                    println!(
                        "Finished opening node name: {} from path: {}",
                        node_name,
                        work_dir.clone().to_str().unwrap()
                    );
                }
                // if failed, the want repository maybe a new repository, try to git clone the repository
                Err(_) => {
                    let url = book_config.remote.clone().unwrap_or_default();
                    match Repository::clone(&url, work_dir.clone()) {
                        Ok(_) => {
                            println!(
                                "Finished cloning node name: {} from url: {}",
                                node_name, url
                            );
                        }
                        Err(e) => return Err(anyhow!("failed to clone : {}", e)),
                    };
                }
            }
            // check the git clone effect
            // crate::utils::command_ls(work_dir.to_str().unwrap());
        }
        Ok(())
    }
}
