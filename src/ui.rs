use crate::model::{SearchMode, SearchState, Tab};
use crate::theme::{self, Theme};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs};
use ratatui::Frame;

pub fn draw(
    f: &mut Frame,
    tabs: &[Tab],
    active: usize,
    search: &SearchState,
    table_state: &mut TableState,
    spinner: Option<&str>,
) {
    let t = theme::current();
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(t.bg)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(if search.active || !search.text.is_empty() { 3 } else { 1 }),
        ])
        .split(area);

    draw_tabs(f, chunks[0], tabs, active, spinner, t);
    draw_binds(f, chunks[1], &tabs[active], search, table_state, spinner, t);
    draw_footer(f, chunks[2], search, t);
}

fn draw_tabs(f: &mut Frame, area: Rect, tabs: &[Tab], active: usize, spinner: Option<&str>, t: &Theme) {
    let titles: Vec<Line> = tabs
        .iter()
        .map(|tab| {
            match spinner {
                Some(frame) if tab.app == "Neovim" && tab.sections.is_empty() => {
                    Line::from(format!("{} {}", tab.app, frame))
                }
                _ => Line::from(tab.app.clone()),
            }
        })
        .collect();
    let widget = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.border))
                .style(Style::default().bg(t.bg))
                .title(" keybinds "),
        )
        .select(active)
        .highlight_style(Style::default().fg(t.accent_text).add_modifier(Modifier::BOLD))
        .style(Style::default().fg(t.text_3).bg(t.bg));
    f.render_widget(widget, area);
}

fn draw_binds(
    f: &mut Frame,
    area: Rect,
    tab: &Tab,
    search: &SearchState,
    table_state: &mut TableState,
    spinner: Option<&str>,
    t: &Theme,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(t.border))
        .style(Style::default().bg(t.bg));

    if tab.app == "Neovim" && tab.sections.is_empty() {
        let msg = Paragraph::new(format!(
            "{} loading nvim keymaps (headless)...",
            spinner.unwrap_or("")
        ))
        .style(Style::default().fg(t.text_3).bg(t.bg))
        .block(block);
        f.render_widget(msg, area);
        return;
    }

    let filtered = tab.filtered_by(search.mode, crate::model::split_at_tab(search.text).1);
    let rows: Vec<Row> = filtered
        .iter()
        .map(|(section, bind)| {
            Row::new(vec![
                Cell::from(Span::styled(section.to_string(), Style::default().fg(t.text_3))),
                Cell::from(Span::styled(
                    bind.keys.clone(),
                    Style::default().fg(t.accent_text).add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(bind.action.clone(), Style::default().fg(t.text))),
            ])
        })
        .collect();

    if rows.is_empty() {
        let msg = Paragraph::new("no results")
            .style(Style::default().fg(t.text_3).bg(t.bg))
            .block(block);
        f.render_widget(msg, area);
        return;
    }

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(25),
            Constraint::Percentage(55),
        ],
    )
    .header(Row::new(vec!["Section", "Key", "Action"]).style(Style::default().fg(t.text_2).add_modifier(Modifier::BOLD)))
    .row_highlight_style(Style::default().bg(t.surface).add_modifier(Modifier::BOLD))
    .highlight_symbol("▍ ")
    .style(Style::default().bg(t.bg))
    .block(block);

    f.render_stateful_widget(table, area, table_state);
}

fn draw_footer(f: &mut Frame, area: Rect, search: &SearchState, t: &Theme) {
    if search.active || !search.text.is_empty() {
        let label = match (search.mode, search.active) {
            (SearchMode::Fuzzy, true) => "search> ",
            (SearchMode::Fuzzy, false) => "filter: ",
            (SearchMode::WhichKey, true) => "which-key> ",
            (SearchMode::WhichKey, false) => "which-key: ",
        };
        let text = format!("{label}{}", search.text);
        let p = Paragraph::new(text).style(Style::default().fg(t.accent_text).bg(t.bg)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.border))
                .style(Style::default().bg(t.bg)),
        );
        f.render_widget(p, area);
    } else {
        let hint = Paragraph::new(
            " h/l switch tab · j/k navigate · Ctrl+d/u half page · gg/G top/bottom · / search · w which-key · q quit ",
        )
            .style(Style::default().fg(t.text_4).bg(t.bg));
        f.render_widget(hint, area);
    }
}
