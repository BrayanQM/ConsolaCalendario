use crate::event::{Event, EventHandler};
use crate::ui;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub due: chrono::DateTime<chrono::Local>,
    pub url: Option<String>,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PanelMode {
    Help,
    Description,
}

#[derive(Clone, Debug)]
pub struct App {
    pub tasks: Vec<Task>,
    pub selected_index: Option<usize>, // index in flat list across columns
    pub panel_mode: PanelMode,
    pub show_delete_prompt: bool,
    pub delete_input: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tasks: vec![],
            selected_index: None,
            panel_mode: PanelMode::Help,
            show_delete_prompt: false,
            delete_input: String::new(),
            should_quit: false,
        }
    }
}

impl App {
    pub fn days_left(&self, t: &Task) -> i64 {
        let now = chrono::Local::now();
        (t.due.date_naive() - now.date_naive()).num_days()
    }

    pub fn classify(&self, t: &Task) -> Column {
        let d = self.days_left(t);
        if d <= 2 { Column::Red } else if d <= 6 { Column::Yellow } else { Column::Green }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Column { Green, Yellow, Red }

pub async fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::default();
    // TODO: load tasks from ICS
    let mut events = EventHandler::new(Duration::from_millis(250));
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;
        match events.next().await? {
            Event::Tick => {}
            Event::Key(key) => handle_key(&mut app, key),
            Event::Resize(_, _) => {}
        }
        if app.should_quit { break; }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    if app.show_delete_prompt {
        match key.code {
            KeyCode::Char(c) => app.delete_input.push(c),
            KeyCode::Backspace => { app.delete_input.pop(); }
            KeyCode::Enter => {
                if app.delete_input.trim().eq_ignore_ascii_case("delete") {
                    // TODO: remove selected task
                    app.show_delete_prompt = false;
                    app.delete_input.clear();
                } else {
                    app.show_delete_prompt = false;
                    app.delete_input.clear();
                }
            }
            KeyCode::Esc => { app.show_delete_prompt = false; app.delete_input.clear(); }
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Esc => { app.panel_mode = PanelMode::Help; app.selected_index = None; }
        KeyCode::Enter => {
            if app.selected_index.is_some() {
                app.panel_mode = PanelMode::Description;
                // TODO: open delete prompt? spec says Enter on task -> input "delete"
                app.show_delete_prompt = true;
                app.delete_input.clear();
            }
        }
        KeyCode::Char('o') | KeyCode::Char('O') => {
            if let Some(idx) = app.selected_index {
                if let Some(t) = app.tasks.get(idx) {
                    if let Some(url) = &t.url { let _ = open::that(url); }
                }
            }
        }
        KeyCode::Up | KeyCode::Char('k') => { /* TODO: navigate */ }
        KeyCode::Down | KeyCode::Char('j') => { /* TODO: navigate */ }
        KeyCode::Left | KeyCode::Char('h') => { /* TODO: navigate */ }
        KeyCode::Right | KeyCode::Char('l') => { /* TODO: navigate */ }
        _ => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Enter) {
                if let Some(idx) = app.selected_index {
                    if let Some(t) = app.tasks.get(idx) {
                        if let Some(url) = &t.url { let _ = open::that(url); }
                    }
                }
            }
        }
    }
}
