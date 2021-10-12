use select::document::Document;
use select::predicate::{Class, Name};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender};
use threadpool::ThreadPool;

pub struct EntriesCache {
    cache: HashMap<String, Vec<String>>,
    ongoing_requests: HashSet<String>,
    threadpool: ThreadPool,
    rx: Receiver<(String, Vec<String>)>,
    tx: Sender<(String, Vec<String>)>,
}

impl EntriesCache {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            cache: HashMap::new(),
            ongoing_requests: HashSet::new(),
            threadpool: ThreadPool::new(num_cpus::get()),
            rx,
            tx,
        }
    }

    pub fn get_repo_entries(&mut self, url: &str) -> Option<&Vec<String>> {
        // Check if we received the response for previous requests
        while let Ok((url, entries)) = self.rx.try_recv() {
            self.ongoing_requests.remove(&url);
            self.cache.insert(url, entries);
        }

        if !self.cache.contains_key(url) && !self.ongoing_requests.contains(url) {
            self.ongoing_requests.insert(url.to_string());

            let tx = self.tx.clone();
            let url = url.to_string();
            self.threadpool.execute(move || {
                let entries = scrape_github_repo(&url);
                tx.send((url, entries)).unwrap();
            });
        }

        self.cache.get(url)
    }
}

fn scrape_github_repo(url: &str) -> Vec<String> {
    let res = reqwest::blocking::get(url).unwrap();
    let document = Document::from_read(res).unwrap();
    let table = document
        .find(Class("Details-content--hidden-not-important"))
        .next()
        .unwrap();

    table
        .children()
        .skip(3)
        .step_by(2)
        .map(|row| {
            // you can check whether an entry is a file or a directory
            // by checking if the svg icon has .octicon-file or .octicon-file-directory
            let is_file = row.find(Class("octicon-file")).next().is_some();
            let end = if is_file { "" } else { "/" };
            row.find(Name("a")).next().unwrap().text() + end
        })
        .collect()
}
