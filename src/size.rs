use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    size: usize,
}
pub fn get_repo_size(url: &str) -> usize {
    let client = Client::builder()
        .user_agent("templa-rs, https://github.com/IEEE-VIT/templa-rs")
        .build()
        .unwrap();
    let res: Response = client
        .get(format!(
            "https://api.github.com/repos{}",
            &url["https://github.com".len()..]
        ))
        .send()
        .unwrap()
        .json()
        .unwrap();
    res.size
}
