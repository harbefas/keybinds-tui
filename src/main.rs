mod cli;
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
use model::{SearchMode, Tab};
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
    search_mode: SearchMode,
    table: TableState,
    nvim_rx: Option<Receiver<Tab>>,
    nvim_idx: usize,
    spinner_frame: usize,
    pending_g: bool,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

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
    let nvim_rx = sources::nvim::spawn();

    if cli::is_non_interactive(&args) {
        // No UI to keep responsive here, so just block for the nvim dump
        // instead of polling it in the background.
        if let Ok(tab) = nvim_rx.recv() {
            tabs[nvim_idx] = tab;
        }
        tabs.extend(config::load_user_tabs());
        return cli::run(&args, &tabs);
    }

    let nvim_rx = Some(nvim_rx);
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
        search_mode: SearchMode::Fuzzy,
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

/// The part of `app.search` actually used to filter rows — with a leading
/// `@tab` tab-selector token (see `sync_at_tab`) stripped off.
fn filter_text(app: &App) -> &str {
    model::split_at_tab(&app.search).1
}

fn row_count(app: &App) -> usize {
    app.tabs[app.active].filtered_by(app.search_mode, filter_text(app)).len()
}

/// While searching, `@token` at the start jumps to the first tab whose name
/// or alias starts with that token, so e.g. typing "@vim scroll" hops to
/// Neovim/Tridactyl and filters for "scroll" within it.
fn sync_at_tab(app: &mut App) {
    let Some(token) = model::split_at_tab(&app.search).0 else {
        return;
    };
    let token = token.to_lowercase();
    if let Some(idx) = app.tabs.iter().position(|t| {
        t.app.to_lowercase().starts_with(&token)
            || t.aliases.iter().any(|a| a.to_lowercase().starts_with(&token))
    }) {
        app.active = idx;
    }
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
                &model::SearchState {
                    text: &app.search,
                    active: app.searching,
                    mode: app.search_mode,
                },
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
                        sync_at_tab(app);
                        reset_selection(app);
                    }
                    KeyCode::Char(c) => {
                        app.search.push(c);
                        sync_at_tab(app);
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
                    app.search_mode = SearchMode::Fuzzy;
                    app.search.clear();
                }
                KeyCode::Char('w') => {
                    app.searching = true;
                    app.search_mode = SearchMode::WhichKey;
                    app.search.clear();
                }
                _ => {}
            }
        }
    }
}
