use crate::model::{Bind, Section, Tab};
use std::collections::BTreeMap;
use std::fs;

/// Parses `bindd`/`bind` lines straight out of every *.conf under ~/.config/hypr.
/// `bindd = MODS, KEY, Description, exec, cmd` -> real description.
/// `bind  = MODS, KEY, dispatcher, arg`        -> falls back to "dispatcher arg".
pub fn load() -> Tab {
    let dir = dirs_hypr();
    let mut grouped: BTreeMap<String, Vec<Bind>> = BTreeMap::new();

    let mut files: Vec<_> = fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("conf"))
        .collect();
    files.sort();

    for path in files {
        let group = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("misc")
            .to_string();
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        for line in content.lines() {
            let line = line.trim();
            if let Some(bind) = parse_bind_line(line) {
                grouped.entry(group.clone()).or_default().push(bind);
            }
        }
    }

    let sections = grouped
        .into_iter()
        .filter(|(_, binds)| !binds.is_empty())
        .map(|(name, binds)| Section { name, binds })
        .collect();

    Tab {
        app: "Hyprland".into(),
        window_class: &[],
        sections,
    }
}

fn parse_bind_line(line: &str) -> Option<Bind> {
    let (directive, rest) = line.split_once('=')?;
    let directive = directive.trim();
    if directive != "bind" && directive != "bindd" && directive != "bindm" {
        return None;
    }
    let parts: Vec<&str> = rest.splitn(if directive == "bindd" { 5 } else { 4 }, ',')
        .map(|p| p.trim())
        .collect();

    if directive == "bindd" && parts.len() >= 5 {
        let mods = parts[0];
        let key = parts[1];
        let desc = parts[2];
        if desc.is_empty() {
            return None;
        }
        return Some(Bind {
            keys: fmt_keys(mods, key),
            action: desc.to_string(),
        });
    }

    if parts.len() >= 3 {
        let mods = parts[0];
        let key = parts[1];
        let action = parts[2..].join(" ");
        return Some(Bind {
            keys: fmt_keys(mods, key),
            action,
        });
    }
    None
}

fn fmt_keys(mods: &str, key: &str) -> String {
    if mods.is_empty() {
        key.to_string()
    } else {
        format!("{} + {}", mods.replace(' ', " + "), key)
    }
}

fn dirs_hypr() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    std::path::PathBuf::from(home).join(".config/hypr")
}
