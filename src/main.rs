mod config;
mod focus;
mod model;
mod sources;
mod theme;
mod ui;

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use model::Tab;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::TableState;
use ratatui::Terminal;
use std::io;
use std::sync::mpsc::Receiver;
use std::time::Duration;

const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const HALF_PAGE: isize = 10;

struct App {
    tabs: Vec<Tab>,
    active: usize,
    search: String,
    searching: bool,
    table: TableState,
    nvim_rx: Option<Receiver<Tab>>,
    nvim_idx: usize,
    spinner_frame: usize,
    pending_g: bool,
}

fn main() -> Result<()> {
    let mut tabs = vec![
        sources::hypr::load(),
        sources::herdr::load(),
        sources::tridactyl::load(),
        sources::spotify_player::load(),
        sources::lazygit::load(),
        sources::yazi::load(),
        sources::glow::load(),
        sources::tuicr::load(),
        sources::nvim::loading_tab(),
    ];
    let nvim_idx = tabs.len() - 1;
    let nvim_rx = Some(sources::nvim::spawn());
    tabs.extend(config::load_user_tabs());

    let guessed = focus::guess_app(&tabs);
    let active = guessed
        .and_then(|name| tabs.iter().position(|t| t.app == name))
        .unwrap_or(0);

    let mut table = TableState::default();
    table.select(Some(0));

    let mut app = App {
        tabs,
        active,
        search: String::new(),
        searching: false,
        table,
        nvim_rx,
        nvim_idx,
        spinner_frame: 0,
        pending_g: false,
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn row_count(app: &App) -> usize {
    app.tabs[app.active].filtered(&app.search).len()
}

fn reset_selection(app: &mut App) {
    app.table.select(if row_count(app) == 0 { None } else { Some(0) });
}

fn move_selection(app: &mut App, delta: isize) {
    let len = row_count(app);
    if len == 0 {
        app.table.select(None);
        return;
    }
    let current = app.table.selected().unwrap_or(0) as isize;
    let next = (current + delta).rem_euclid(len as isize) as usize;
    app.table.select(Some(next));
}

fn scroll_page(app: &mut App, delta: isize) {
    let len = row_count(app);
    if len == 0 {
        app.table.select(None);
        return;
    }
    let current = app.table.selected().unwrap_or(0) as isize;
    let next = (current + delta).clamp(0, len as isize - 1) as usize;
    app.table.select(Some(next));
}

fn jump_to(app: &mut App, index: usize) {
    let len = row_count(app);
    if len == 0 {
        app.table.select(None);
        return;
    }
    app.table.select(Some(index.min(len - 1)));
}

fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // Poll the nvim dump thread until it lands, replacing the loading tab in place.
        if let Some(rx) = &app.nvim_rx {
            if let Ok(tab) = rx.try_recv() {
                app.tabs[app.nvim_idx] = tab;
                app.nvim_rx = None;
                if app.active == app.nvim_idx {
                    reset_selection(app);
                }
            }
        }

        let loading = app.nvim_rx.is_some();
        let spinner = if loading {
            Some(SPINNER[app.spinner_frame % SPINNER.len()])
        } else {
            None
        };

        terminal.draw(|f| {
            ui::draw(
                f,
                app.tabs.as_slice(),
                app.active,
                &app.search,
                app.searching,
                &mut app.table,
                spinner,
            )
        })?;

        if !event::poll(Duration::from_millis(100))? {
            if loading {
                app.spinner_frame = app.spinner_frame.wrapping_add(1);
            }
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if app.searching {
                match key.code {
                    KeyCode::Esc => {
                        app.searching = false;
                        app.search.clear();
                        reset_selection(app);
                    }
                    KeyCode::Enter => app.searching = false,
                    KeyCode::Backspace => {
                        app.search.pop();
                        reset_selection(app);
                    }
                    KeyCode::Char(c) => {
                        app.search.push(c);
                        reset_selection(app);
                    }
                    _ => {}
                }
                continue;
            }

            let was_pending_g = app.pending_g;
            app.pending_g = false;

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                    app.active = (app.active + 1) % app.tabs.len();
                    reset_selection(app);
                }
                KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                    app.active = (app.active + app.tabs.len() - 1) % app.tabs.len();
                    reset_selection(app);
                }
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    scroll_page(app, HALF_PAGE)
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    scroll_page(app, -HALF_PAGE)
                }
                KeyCode::Down | KeyCode::Char('j') => move_selection(app, 1),
                KeyCode::Up | KeyCode::Char('k') => move_selection(app, -1),
                KeyCode::Char('g') => {
                    if was_pending_g {
                        jump_to(app, 0);
                    } else {
                        app.pending_g = true;
                    }
                }
                KeyCode::Char('G') => jump_to(app, row_count(app).saturating_sub(1)),
                KeyCode::Char('/') => {
                    app.searching = true;
                    app.search.clear();
                }
                _ => {}
            }
        }
    }
}
