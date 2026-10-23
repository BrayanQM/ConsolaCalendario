use crate::app::{App, PanelMode};
use crate::util::{format_date, format_time_12h};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Row, Table, Cell},
    Frame,
};

fn get_theme() -> Theme {
    Theme {
        border: Color::Cyan,
        title: Color::Yellow,
        selected_border: Color::Magenta,
        selected_fg: Color::White,
        text: Color::Gray,
        header_green: Color::Green,
        header_yellow: Color::Yellow,
        header_red: Color::Red,
        bg_sel: Color::Rgb(40, 40, 60),
    }
}

#[derive(Clone)]
struct Theme {
    border: Color,
    title: Color,
    selected_border: Color,
    selected_fg: Color,
    text: Color,
    header_green: Color,
    header_yellow: Color,
    header_red: Color,
    bg_sel: Color,
}

pub fn render(f: &mut Frame, app: &mut App) {
    let theme = get_theme();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    render_left(f, chunks[0], app, &theme);
    render_right(f, chunks[1], app, &theme);

    if app.show_delete_prompt {
        let area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, area);
        let block = Block::default()
            .title(Span::styled("Eliminar tarea - escribe 'delete' y presiona Enter", Style::default().fg(theme.title)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.selected_border));
        let p = Paragraph::new(app.delete_input.as_str()).block(block);
        f.render_widget(p, area);
    }
}

fn render_left(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)])
        .split(area);

    let mut green_items: Vec<ListItem> = vec![];
    let mut yellow_items: Vec<ListItem> = vec![];
    let mut red_items: Vec<ListItem> = vec![];

    for (idx, t) in app.tasks.iter().enumerate() {
        let days = app.days_left(t);
        let selected = app.selected_index == Some(idx);
        let marker = if selected { "▸ " } else { "  " };
        let style = if selected { Style::default().fg(theme.selected_fg).add_modifier(Modifier::BOLD) } else { Style::default().fg(theme.text) };
        let url_text = t.url.as_deref().unwrap_or("-");
        let item = ListItem::new(vec![
            Line::from(Span::styled(format!("{}{}", marker, t.title), style.add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(format!("Fecha: {}  Hora: {}", format_date(&t.due), format_time_12h(&t.due)), style)),
            Line::from(Span::styled(format!("Materia: {}", t.subject), style)),
            Line::from(Span::styled(format!("URL: {}", url_text), style)),
            Line::from(""),
        ]).style(if selected { Style::default().bg(theme.bg_sel) } else { Style::default() });
        if days <= 2 {
            red_items.push(item);
        } else if days <= 6 {
            yellow_items.push(item);
        } else {
            green_items.push(item);
        }
    }

    let block_g = Block::default().title(Span::styled("TIEMPO VERDE", Style::default().fg(theme.header_green))).borders(Borders::ALL).border_style(Style::default().fg(theme.border));
    let block_y = Block::default().title(Span::styled("TIEMPO AMARILLO", Style::default().fg(theme.header_yellow))).borders(Borders::ALL).border_style(Style::default().fg(theme.border));
    let block_r = Block::default().title(Span::styled("TIEMPO ROJO", Style::default().fg(theme.header_red))).borders(Borders::ALL).border_style(Style::default().fg(theme.border));
    f.render_widget(List::new(green_items).block(block_g), cols[0]);
    f.render_widget(List::new(yellow_items).block(block_y), cols[1]);
    f.render_widget(List::new(red_items).block(block_r), cols[2]);
}

fn render_right(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    if app.panel_mode == PanelMode::Help || app.selected_index.is_none() {
        let block = Block::default().title(Span::styled("COMMAND KEYBOARD", Style::default().fg(theme.title))).borders(Borders::ALL).border_style(Style::default().fg(theme.border));
        let rows = vec![
            Row::new(vec![Cell::from("↑/↓/←/→ o h/j/k/l"), Cell::from("Navegar entre tareas")]),
            Row::new(vec![Cell::from("Enter"), Cell::from("Ver descripción / Eliminar (escribir delete)")]),
            Row::new(vec![Cell::from("o / Ctrl+Enter"), Cell::from("Abrir URL en navegador")]),
            Row::new(vec![Cell::from("Esc"), Cell::from("Volver a ayuda / limpiar selección")]),
            Row::new(vec![Cell::from("q / Ctrl+C"), Cell::from("Salir")]),
        ];
        let table = Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .header(Row::new(vec![Cell::from(Span::styled("Comando", Style::default().add_modifier(Modifier::BOLD))), Cell::from(Span::styled("Descripción", Style::default().add_modifier(Modifier::BOLD)))]))
            .block(block);
        f.render_widget(table, area);
    } else {
        let block = Block::default().title(Span::styled("DESCRIPCIÓN", Style::default().fg(theme.title))).borders(Borders::ALL).border_style(Style::default().fg(theme.selected_border));
        let mut text = String::new();
        if let Some(idx) = app.selected_index {
            if let Some(t) = app.tasks.get(idx) {
                text.push_str(&format!("Tarea: {}\nMateria: {}\nFecha: {} {}\nURL: {}\n\n{}\n", t.title, t.subject, format_date(&t.due), format_time_12h(&t.due), t.url.as_deref().unwrap_or("-"), t.description));
            }
        }
        let p = Paragraph::new(text).block(block).wrap(ratatui::widgets::Wrap { trim: true }).style(Style::default().fg(theme.text));
        f.render_widget(p, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
