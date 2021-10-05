use std::path::Path;

//Git Command
use git2::{build::RepoBuilder, Error, FetchOptions, RemoteCallbacks, Repository};
use progress_bar::{
    color::{Color, Style},
    progress_bar::ProgressBar,
};

use crate::size::get_repo_size;

pub fn git_clone(proj_name: &str, url: String) -> Result<Repository, Error> {
    let repo_size = get_repo_size(&url);

    let mut progress_bar = ProgressBar::new(100);
    progress_bar.set_action("Loading", Color::Blue, Style::Bold);

    let mut cb = RemoteCallbacks::new();

    cb.transfer_progress(|stats| {
        let percentage_done = (stats.received_bytes() / 1000000) as f64  / (repo_size / 1000)  as f64;
        progress_bar.set_progression((percentage_done * 100 as f64) as usize);
        true
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    let repo = RepoBuilder::new()
        .fetch_options(fo)
        .clone(&url, Path::new(&proj_name));
    match repo {
        Ok(_) => progress_bar.set_progression(100),
        Err(_) => {}
    };
    repo
}
