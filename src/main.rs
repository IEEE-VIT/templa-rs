use clap::{load_yaml, App};
use reqwest::blocking::Client;
use serde_json::Value;

mod command;
mod models;
mod search;
mod size;
mod tui;
mod app;

const SUBMODULES_URL: &str = "https://api.github.com/repos/IEEE-VIT/templa-rs/contents/submodules.json";

fn fetch_submodules() -> Result<Vec<models::structs::Submodule>, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("templa-rs, https://github.com/IEEE-VIT/templa-rs")
        .build()?;
    let response: Value = client
        .get(SUBMODULES_URL)
        .header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
        )
        .send()?
        .json()?;
    let encoded_body = response["content"].as_str().unwrap();
    let decoded_body = base64::decode(encoded_body.replace('\n', "")).unwrap();
    Ok(serde_json::from_slice(&decoded_body)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("../commands.yml");
    let matches = App::from(yaml).get_matches();
    let mut proj = String::new();

    if let Some(proj_name) = matches.value_of("name") {
        proj = String::from(proj_name);
    }

    let submodules = match fetch_submodules() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not load submodules from GitHub : {}", e);
            return Ok(());
        }
    };

    tui::render_tui(&proj, submodules, matches.value_of("first-query"), matches.value_of("template-type"));
    Ok(())
}
