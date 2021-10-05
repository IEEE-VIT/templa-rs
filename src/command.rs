use std::path::Path;

//Git Command
use git2::{build::RepoBuilder, Error, FetchOptions, RemoteCallbacks, Repository};
use progress_bar::{
    color::{Color, Style},
    progress_bar::ProgressBar,
};

use crate::size::get_repo_size;

pub fn git_clone(proj_name: &str, url: String) -> Result<Repository, Error> {
    println!("PROJ NAME = {}", &proj_name);
    let repo_size = get_repo_size(&url);

    let mut progress_bar = ProgressBar::new(repo_size / 1000);

    progress_bar.set_action("Loading", Color::Blue, Style::Bold);

    let mut cb = RemoteCallbacks::new();

    cb.transfer_progress(|stats| {
        progress_bar.set_progression(stats.received_bytes() / 1000000);
        true
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    let repo = RepoBuilder::new()
        .fetch_options(fo)
        .clone(&url, Path::new(&proj_name));
    match repo {
        Ok(_) => progress_bar.set_progression(&repo_size / 1000),
        Err(_) => {}
    };
    repo
}
