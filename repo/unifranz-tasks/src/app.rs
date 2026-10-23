use crate::config::Config;
use crate::event::{Event, EventHandler};
use crate::ical;
use crate::state::Seen;
use crate::ui;
use crate::util::clean_description;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
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
    pub selected_index: Option<usize>,
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
}

pub async fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::default();
    if let Ok(cfg) = Config::load() {
        if let Ok(ics_tasks) = ical::fetch_and_parse(&cfg.ics.url) {
            let seen = Seen::load().unwrap_or_default();
            for it in ics_tasks {
                if seen.ids.contains(&it.uid) { continue; }
                app.tasks.push(Task {
                    id: it.uid.clone(),
                    title: it.title.clone(),
                    subject: it.subject.clone().unwrap_or_else(|| "Sin materia".to_string()),
                    due: it.due,
                    url: it.url.clone(),
                    description: clean_description(&it.description),
                });
            }
            app.tasks.sort_by_key(|t| t.due);
        }
    }
    let events = EventHandler::new(Duration::from_millis(150));
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;
        match events.next_blocking() {
            Ok(Some(Event::Key(key))) => {
                if app.show_delete_prompt {
                    match key.code {
                        KeyCode::Char(c) => app.delete_input.push(c),
                        KeyCode::Backspace => { app.delete_input.pop(); }
                        KeyCode::Enter => {
                            if app.delete_input.trim().eq_ignore_ascii_case("delete") {
                                if let Some(idx) = app.selected_index {
                                    if idx < app.tasks.len() {
                                        let tid = app.tasks[idx].id.clone();
                                        let mut seen = Seen::load().unwrap_or_default();
                                        seen.ids.insert(tid);
                                        let _ = seen.save();
                                        app.tasks.remove(idx);
                                        if app.tasks.is_empty() {
                                            app.selected_index = None;
                                            app.panel_mode = PanelMode::Help;
                                        } else if idx >= app.tasks.len() {
                                            app.selected_index = Some(app.tasks.len()-1);
                                        }
                                    }
                                }
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
                    continue;
                }
                // Ctrl+C
                if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c')) {
                    app.should_quit = true;
                    break;
                }
                // Ctrl+Enter
                if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Enter) {
                    if let Some(idx) = app.selected_index {
                        if let Some(t) = app.tasks.get(idx) {
                            if let Some(url) = &t.url { let _ = open::that(url); }
                        }
                    } else if !app.tasks.is_empty() {
                        if let Some(t) = app.tasks.get(0) {
                            if let Some(url) = &t.url { let _ = open::that(url); }
                        }
                    }
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => { app.should_quit = true; break; }
                    KeyCode::Esc => { app.panel_mode = PanelMode::Help; app.selected_index = None; }
                    KeyCode::Enter => {
                        if app.selected_index.is_some() && !app.tasks.is_empty() {
                            app.panel_mode = PanelMode::Description;
                            app.show_delete_prompt = true;
                            app.delete_input.clear();
                        }
                    }
                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        if let Some(idx) = app.selected_index {
                            if let Some(t) = app.tasks.get(idx) {
                                if let Some(url) = &t.url { let _ = open::that(url); }
                            }
                        } else if !app.tasks.is_empty() {
                            if let Some(t) = app.tasks.get(0) {
                                if let Some(url) = &t.url { let _ = open::that(url); }
                            }
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.tasks.is_empty() { continue; }
                        match app.selected_index {
                            None => app.selected_index = Some(0),
                            Some(i) => if i == 0 { app.selected_index = Some(app.tasks.len()-1); } else { app.selected_index = Some(i-1); }
                        }
                        app.panel_mode = PanelMode::Description;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.tasks.is_empty() { continue; }
                        match app.selected_index {
                            None => app.selected_index = Some(0),
                            Some(i) => if i >= app.tasks.len()-1 { app.selected_index = Some(0); } else { app.selected_index = Some(i+1); }
                        }
                        app.panel_mode = PanelMode::Description;
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if app.tasks.is_empty() { continue; }
                        match app.selected_index {
                            None => app.selected_index = Some(0),
                            Some(i) => if i == 0 { app.selected_index = Some(app.tasks.len()-1); } else { app.selected_index = Some(i-1); }
                        }
                        app.panel_mode = PanelMode::Description;
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if app.tasks.is_empty() { continue; }
                        match app.selected_index {
                            None => app.selected_index = Some(0),
                            Some(i) => if i >= app.tasks.len()-1 { app.selected_index = Some(0); } else { app.selected_index = Some(i+1); }
                        }
                        app.panel_mode = PanelMode::Description;
                    }
                    _ => {}
                }
            }
            Ok(Some(Event::Tick)) => {}
            Ok(Some(Event::Resize(_, _))) => {}
            Ok(Some(Event::Mouse(_))) => {}
            Ok(None) => {}
            Err(_) => break,
        }
    }
    Ok(())
}
