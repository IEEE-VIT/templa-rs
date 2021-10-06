use crate::app::App;
use crate::command;
use crate::models::{enums, structs::Submodule};
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

pub fn render_tui(proj_name: &str, submodules: Vec<Submodule>, optional_first_query: Option<&str>, optional_tag: Option<&str>,) {
    enable_raw_mode().expect("can run in raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // App state container
    let mut full_first_query = String::new();
    
    if let Some(tag_raw) = optional_tag {
        full_first_query.push_str("tag:");
        full_first_query.push_str(tag_raw);
        full_first_query.push(' ');
    }
    
    if let Some(first_query) = optional_first_query {
        full_first_query.push_str(first_query);
    }

    let mut app = App::new(submodules, &full_first_query);

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
                    tx.send(enums::Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(enums::Event::Tick).is_ok() {
    	        last_tick = Instant::now();
            }
        }
    });
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    let mut template_list_state = ListState::default();
    template_list_state.select(Some(0));

    loop {
        app.search();

        terminal
            .draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Percentage(15),
                            Constraint::Percentage(80),
                        ].as_ref()
                    )
                    .split(size);

                let template_list = render_template_list(&app.filtered_submodules, app.skin_color);
                let template_search = render_search_bar(&app.current_query, app.skin_color);

                rect.render_widget(template_search, chunks[0]);
                rect.render_stateful_widget(template_list, chunks[1], &mut template_list_state);
            })
            .unwrap();

        match rx.recv().unwrap() {
            enums::Event::Input(event) => match (event.code, event.modifiers) {
                (KeyCode::Char(key), KeyModifiers::NONE) => {
                    app.current_query.push(key);
                }
                (KeyCode::Char(key), KeyModifiers::SHIFT) => {
                    app.current_query.push(key.to_ascii_uppercase());
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    app.current_query.pop();
                }
                (KeyCode::Backspace, KeyModifiers::CONTROL) => {
                    app.current_query.clear();
                }
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    disable_raw_mode().unwrap();
                    execute!(terminal.backend_mut(), LeaveAlternateScreen,).unwrap();
                    terminal.show_cursor().unwrap();
                    break;
                }
                (KeyCode::Down, _) => {
                    if let Some(selected) = template_list_state.selected() {
                        let templates_length = app.filtered_submodules.len();
                        if selected >= templates_length - 1 {
                            template_list_state.select(Some(0));
                        } else {
                            template_list_state.select(Some(selected + 1));
                        }
                    }
                }
                (KeyCode::Up, _) => {
                    if let Some(selected) = template_list_state.selected() {
                        let templates_length = app.filtered_submodules.len();
                        if selected > 0 {
                            template_list_state.select(Some(selected - 1));
                        } else {
                            template_list_state.select(Some(templates_length - 1));
                        }
                    }
                }
                (KeyCode::Enter, _) => {
                    if template_list_state.selected().is_some() {
                        let templates = &app.filtered_submodules;
                        let selected_template = templates
                            .get(
                                template_list_state
                                    .selected()
                                    .expect("there is always a selected template"),
                            )
                            .unwrap();

                        disable_raw_mode().unwrap();
                        execute!(terminal.backend_mut(), LeaveAlternateScreen,).unwrap();
                        let proj_name = if proj_name.to_string().is_empty() { selected_template.name.to_string() } else { proj_name.to_string() };
                        println!("\nCloning {}..\n", proj_name);
                        let output =
                            command::git_clone(&proj_name, selected_template.url.to_string());
                        terminal.show_cursor().unwrap();
                        match output {
                            Ok(_) => {
                                println!("\nCloned the repo successfully")
                            }
                            Err(e) => {
                                println!("{}", e)
                            }
                        }
                        break;
                    }
                }
                _ => {}
            },
            enums::Event::Tick => {}
        }
    }
}

fn render_search_bar(search_query: &str, skin_color: Color) -> Paragraph {
    let mut search_content = String::new();
    search_content.push_str(" / ");
    search_content.push_str(search_query);

    Paragraph::new(Text::from(search_content)).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(skin_color))
    ).style(Style::default().fg(Color::White))
}

fn render_template_list<'a>(filtered_submodules: &[Submodule], skin_color: Color) -> List<'a> {
    let title_span = Span::styled(
        " Available Templates ",
        Style::default().fg(skin_color).add_modifier(Modifier::BOLD)
    );

    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(skin_color))
        .title(title_span)
        .border_type(BorderType::Plain);

    let items: Vec<_> = filtered_submodules
        .iter()
        .map(|submodule| {
            ListItem::new(Spans::from(vec![Span::styled(
                submodule.name.clone(),
                Style::default().fg(Color::White),
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
