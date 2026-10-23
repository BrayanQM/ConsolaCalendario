use crate::app::{App, PanelMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Table, Row, Cell},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    render_left(f, chunks[0], app);
    render_right(f, chunks[1], app);

    if app.show_delete_prompt {
        let area = centered_rect(60, 20, f.area());
        f.render_widget(ratatui::widgets::Clear, area);
        let block = Block::default().title("Eliminar tarea - escribe 'delete'").borders(Borders::ALL);
        let p = Paragraph::new(app.delete_input.as_str()).block(block);
        f.render_widget(p, area);
    }
}

fn render_left(f: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)])
        .split(area);

    // headers
    let h1 = Block::default().title("TIEMPO VERDE").borders(Borders::ALL);
    let h2 = Block::default().title("TIEMPO AMARILLO").borders(Borders::ALL);
    let h3 = Block::default().title("TIEMPO ROJO").borders(Borders::ALL);
    f.render_widget(h1.clone(), inset(cols[0], 0, 0, 3, 0));
    f.render_widget(h2.clone(), inset(cols[1], 0, 0, 3, 0));
    f.render_widget(h3.clone(), inset(cols[2], 0, 0, 3, 0));

    // placeholder
    let items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            Span::styled("[TITULO] ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("Ejemplo Tarea"),
        ])),
        ListItem::new(Line::from("Fecha: 25/10/26  11:59 PM")),
        ListItem::new(Line::from("Materia: Programación")),
        ListItem::new(Line::from("URL: https://... (Ctrl+Enter/o)")),
    ];
    let list = List::new(items).block(Block::default().borders(Borders::ALL));
    f.render_widget(list, inset(cols[0], 1, 1, 0, 1));
}

fn render_right(f: &mut Frame, area: Rect, app: &App) {
    if app.panel_mode == PanelMode::Help || app.selected_index.is_none() {
        let block = Block::default().title("COMMAND KEYBOARD").borders(Borders::ALL);
        let rows = vec![
            Row::new(vec![Cell::from("↑/↓/←/→ o h/j/k/l"), Cell::from("Navegar")]),
            Row::new(vec![Cell::from("Enter"), Cell::from("Seleccionar / Eliminar (escribir delete)")]),
            Row::new(vec![Cell::from("o / Ctrl+Enter"), Cell::from("Abrir URL en navegador")]),
            Row::new(vec![Cell::from("Esc"), Cell::from("Volver a ayuda / limpiar selección")]),
            Row::new(vec![Cell::from("q"), Cell::from("Salir")]),
        ];
        let table = Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .header(Row::new(vec![Cell::from("Comando"), Cell::from("Descripción")]).style(Style::default().add_modifier(Modifier::BOLD)))
            .block(block);
        f.render_widget(table, area);
    } else {
        let block = Block::default().title("DESCRIPCIÓN").borders(Borders::ALL);
        let p = Paragraph::new("Descripción limpia en formato .md sin simbolos extra.\n\n(usar Ctrl+Enter / o para abrir URL)").block(block);
        f.render_widget(p, area);
    }
}

fn inset(r: Rect, dy1: u16, dx1: u16, dy2: u16, dx2: u16) -> Rect {
    Rect { x: r.x+dx1, y: r.y+dy1, width: r.width.saturating_sub(dx1+dx2), height: r.height.saturating_sub(dy1+dy2) }
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
