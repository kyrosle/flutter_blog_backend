use std::{collections::HashMap, process::Command};

use flutter_blog_backend::core::{
    git_book_manager::config::{read_book_config::BookConfigs, ReadConfig},
    SPath,
};
use git2::Repository;
use tempfile::tempdir;

fn main() {
    // config path is set under the workspace
    let book_configs =
        BookConfigs::config_read_from_yaml_path("test-config/books-config.yml").unwrap();

    // set the path later
    let temp_dir = tempdir().unwrap();
    let dir = temp_dir.path().to_str().unwrap();
    let dir = SPath::new(dir);

    let mut git_book_manager = HashMap::new();

    for (node_name, book) in book_configs.configs.iter() {
        println!("cloning node name: {} from url: {}", node_name, book.path);
        let url = book.path.clone();
        let dir = dir.clone().push(node_name);
        let repo = match Repository::clone(&url, dir) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone : {}", e),
        };
        git_book_manager.insert(node_name, repo);
    }

    // execute powershell command to show tempdir directories and files
    let output = Command::new("powershell")
        .arg("/c")
        .arg("ls")
        .arg(dir.to_str().unwrap())
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
