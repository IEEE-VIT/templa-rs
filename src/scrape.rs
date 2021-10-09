use reqwest;
use select::document::Document;
use select::predicate::{Class, Name};

pub fn scrape_github_repo(url: &str) -> Option<Vec<String>> {
    let res = reqwest::blocking::get(url).unwrap();
    let document = Document::from_read(res).unwrap();
    let table = document
        .find(Class("Details-content--hidden-not-important"))
        .next()?;

    // you can check whether an entry is a file or a directory
    // by checking for .octicon-file or .octicon-file-directory
    let entries = table
        .children()
        .skip(3)
        .step_by(2)
        .map(|row| row.find(Name("a")).next().unwrap().text())
        .collect();

    Some(entries)
}
