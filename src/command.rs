//Git Command
use std::process::{Command, Output};
use std::str;

pub fn git_clone(proj_name: &str, url: String) -> Output {
    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(proj_name)
        .output()
        .expect("failed to execute process");
    output
}
