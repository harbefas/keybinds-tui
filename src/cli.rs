use crate::model::Tab;
use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn is_non_interactive(args: &[String]) -> bool {
    args.iter()
        .any(|a| a == "--list" || a == "--json" || a == "--conflicts")
}

/// Runs one of the non-interactive modes. `--list` is meant for piping into
/// an external picker (rofi/walker/fzf -dmenu style): one bind per line,
/// "App | Section | Keys | Action".
pub fn run(args: &[String], tabs: &[Tab]) -> Result<()> {
    if args.iter().any(|a| a == "--json") {
        print_json(tabs)
    } else if args.iter().any(|a| a == "--conflicts") {
        print_conflicts(tabs);
        Ok(())
    } else {
        print_list(tabs);
        Ok(())
    }
}

/// Ignores a broken pipe (e.g. the consumer end — `head`, `walker --dmenu`,
/// `fzf` — closed early) instead of panicking, since that's a normal way
/// for a picker consumer to behave, not an error.
fn writeln_or_stop(out: &mut impl Write, line: &str) -> bool {
    match writeln!(out, "{line}") {
        Ok(()) => true,
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => false,
        Err(e) => panic!("{e}"),
    }
}

fn print_list(tabs: &[Tab]) {
    let mut out = io::stdout();
    for tab in tabs {
        for (section, bind) in tab.flat() {
            let line = format!("{} | {} | {} | {}", tab.app, section, bind.keys, bind.action);
            if !writeln_or_stop(&mut out, &line) {
                return;
            }
        }
    }
}

fn print_json(tabs: &[Tab]) -> Result<()> {
    #[derive(serde::Serialize)]
    struct Row<'a> {
        app: &'a str,
        section: &'a str,
        keys: &'a str,
        action: &'a str,
    }

    let rows: Vec<Row> = tabs
        .iter()
        .flat_map(|tab| {
            tab.flat().into_iter().map(move |(section, bind)| Row {
                app: &tab.app,
                section,
                keys: &bind.keys,
                action: &bind.action,
            })
        })
        .collect();

    writeln_or_stop(&mut io::stdout(), &serde_json::to_string_pretty(&rows)?);
    Ok(())
}

/// Informational only: flags the same key string showing up under more than
/// one app. Not scoped by focus context, since two apps can happily reuse a
/// key if they're never focused at the same time — it's just a heads-up.
fn print_conflicts(tabs: &[Tab]) {
    let mut by_key: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for tab in tabs {
        for (_, bind) in tab.flat() {
            by_key
                .entry(bind.keys.to_lowercase())
                .or_default()
                .push((tab.app.clone(), bind.action.clone()));
        }
    }

    let mut collisions: Vec<_> = by_key
        .into_iter()
        .filter(|(_, uses)| {
            let mut apps: Vec<_> = uses.iter().map(|(app, _)| app.as_str()).collect();
            apps.sort_unstable();
            apps.dedup();
            apps.len() > 1
        })
        .collect();
    collisions.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = io::stdout();
    if collisions.is_empty() {
        writeln_or_stop(&mut out, "no cross-app key collisions found");
        return;
    }

    for (key, uses) in collisions {
        if !writeln_or_stop(&mut out, &format!("{key}:")) {
            return;
        }
        for (app, action) in uses {
            if !writeln_or_stop(&mut out, &format!("  {app}: {action}")) {
                return;
            }
        }
    }
}
