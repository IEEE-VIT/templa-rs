//Git Command
use std::process::Command;
use std::str;
use std::io::Stdout;
use crossterm::terminal::disable_raw_mode;
use tui::{Terminal, backend::CrosstermBackend};

pub fn git_clone(proj_name: &str, url: String,terminal : &mut Terminal<CrosstermBackend<Stdout>>){
    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(proj_name)
        .output()
        .expect("failed to execute process");
    disable_raw_mode().unwrap();
    terminal.show_cursor().unwrap();
    if output.status.success() {
        println!("\nCloned {} successfully\n", &proj_name);
    } else {
        println!(
            "\nError Encountered while cloning, {:?}",
            str::from_utf8(&output.stderr)
        );
    }
}