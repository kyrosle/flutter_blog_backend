pub mod git_pull;
use std::process::Command;

pub fn command_ls(path: impl AsRef<std::ffi::OsStr>) {
    let output = Command::new("powershell")
        .arg("/c")
        .arg("ls")
        .arg(path)
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

