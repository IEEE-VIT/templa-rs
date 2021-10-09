use reqwest;
use select::document::Document;
use select::predicate::{Class, Name};

pub fn scrape_github_repo(url: &str) -> Option<Vec<String>> {
    let res = reqwest::blocking::get(url).unwrap();
    let document = Document::from_read(res).unwrap();
    let table = document
        .find(Class("Details-content--hidden-not-important"))
        .next()?;

    let entries = table
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
        .collect();

    Some(entries)
}
