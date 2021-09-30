use clap::{load_yaml, App};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::Command;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Terminal,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}
enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Submodule {
    name: String,
    url: String,
    tags: Vec<String>,
}

const JSON_PATH: &str = "./submodules.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("../commands.yml");
    let matches = App::from(yaml).get_matches();
    enable_raw_mode().expect("can run in raw mode");
    let mut key = String::new();
    if let Some(template_name) = matches.value_of("template-type") {
        key = String::from(template_name);
    }
    let mut input = String::from("react");
    input.truncate(input.len() - 1);

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut template_list_state = ListState::default();
    template_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(80)].as_ref())
                .split(size);
            let template_list = render_pets(key.clone());
            rect.render_stateful_widget(template_list, chunks[0], &mut template_list_state);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    println!("{:?}", template_list_state.selected());
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Down => {
                    if let Some(selected) = template_list_state.selected() {
                        let templates_length = read_json().expect("can fetch template list").len();
                        if selected >= templates_length - 1 {
                            template_list_state.select(Some(0));
                        } else {
                            template_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = template_list_state.selected() {
                        let templates_length = read_json().expect("can fetch template list").len();
                        if selected > 0 {
                            template_list_state.select(Some(selected - 1));
                        } else {
                            template_list_state.select(Some(templates_length - 1));
                        }
                    }
                }
                KeyCode::Enter => {
                    if let Some(_) = template_list_state.selected() {
                        let templates = perform_search(read_json().unwrap(), key.clone())
                            .expect("can fetch template list");
                        let selected_template = templates
                            .get(
                                template_list_state
                                    .selected()
                                    .expect("there is always a selected pet"),
                            )
                            .unwrap();
                        if let Some(proj_name) = matches.value_of("name") {
                            let output = Command::new("git")
                                .arg("clone")
                                .arg(selected_template.url.to_string())
                                .arg(proj_name.to_string())
                                .output()
                                .expect("failed to execute process");
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            if output.status.success() {
                                println!("\nCloned {} successfully\n", selected_template.name);
                            } else {
                                println!(
                                    "\nError Encountered while cloning, {:?}",
                                    str::from_utf8(&output.stderr)
                                );
                            }
                            break;
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}

fn render_pets<'a>(key: String) -> List<'a> {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Available Templates")
        .border_type(BorderType::Plain);

    let template_list = read_json().expect("can fetch json");
    let filtered_template_list = perform_search(template_list, key).expect("can filter json");
    let items: Vec<_> = filtered_template_list
        .iter()
        .map(|submodule| {
            ListItem::new(Spans::from(vec![Span::styled(
                submodule.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(pets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}

fn perform_search(submodules: Vec<Submodule>, key: String) -> Result<Vec<Submodule>, Error> {
    let mut filtered_sm = vec![];
    for submodule in submodules.iter() {
        if submodule.tags.contains(&key) {
            filtered_sm.push(submodule.clone());
        }
    }
    Ok(filtered_sm)
}

fn read_json() -> Result<Vec<Submodule>, Error> {
    let jsonfile = fs::read_to_string(JSON_PATH)?;
    let parsed: Vec<Submodule> = serde_json::from_str(&jsonfile)?;
    Ok(parsed)
}