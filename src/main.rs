use clap::{load_yaml, App};

mod tui;
mod parse_json;
mod command;
mod search;
mod models;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("../commands.yml");
    let matches = App::from(yaml).get_matches();
    let mut key = String::new();
    let mut proj = String::new();
    if let Some(template_name) = matches.value_of("template-type") {
        key = String::from(template_name);
    }
    if let Some(proj_name) = matches.value_of("name") {
        proj = String::from(proj_name);
    }

    tui::render_tui(key, &proj);
    Ok(())
}