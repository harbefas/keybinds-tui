use crate::model::{Bind, Section, Tab};
use std::fs;

/// herdr has no `dump-keybindings` command and its defaults are compiled into
/// the binary (src/ui/keybind_help.rs), so the base table below is hand-copied
/// from that source once. What we *can* read live is ~/.config/herdr/config.toml's
/// [keys] table, which overrides individual defaults — those are merged in.
const DEFAULTS: &[(&str, &str, &str)] = &[
    ("Workspaces", "Prefix + Down", "Next workspace"),
    ("Workspaces", "Prefix + Up", "Previous workspace"),
    ("Workspaces", "Prefix + W", "Workspace picker"),
    ("Workspaces", "Prefix + Shift + N", "New workspace"),
    ("Workspaces", "Prefix + Shift + W", "Rename workspace"),
    ("Workspaces", "Prefix + Shift + D", "Close workspace"),
    ("Tabs", "Prefix + Right", "Next tab"),
    ("Tabs", "Prefix + Left", "Previous tab"),
    ("Tabs", "Prefix + C", "New tab"),
    ("Tabs", "Prefix + Shift + T", "Rename tab"),
    ("Tabs", "Prefix + Shift + X", "Close tab"),
    ("Tabs", "Prefix + 1..9", "Go to tab N"),
    ("Panes", "Prefix + V", "Vertical split"),
    ("Panes", "Prefix + -", "Horizontal split"),
    ("Panes", "Prefix + X", "Close pane"),
    ("Panes", "Prefix + Z", "Zoom (fullscreen) pane"),
    ("Panes", "Prefix + H/J/K/L", "Focus pane left/down/up/right"),
    ("Panes", "Prefix + Tab", "Cycle panes"),
    ("Panes", "Prefix + R", "Resize mode"),
    ("Misc", "Prefix + ?", "Help"),
    ("Misc", "Prefix + B", "Toggle sidebar"),
    ("Misc", "Prefix + G", "Goto (search)"),
    ("Misc", "Prefix + E", "Edit scrollback"),
];

pub fn load() -> Tab {
    let overrides = read_overrides();
    let mut sections: std::collections::BTreeMap<String, Vec<Bind>> = Default::default();

    for (section, keys, action) in DEFAULTS {
        let action_name = action_id(action);
        let keys = overrides
            .get(&action_name)
            .cloned()
            .unwrap_or_else(|| keys.to_string());
        sections
            .entry(section.to_string())
            .or_default()
            .push(Bind {
                keys,
                action: action.to_string(),
            });
    }

    Tab {
        app: "Herdr".into(),
        window_class: &[],
        aliases: &["tmux", "multiplexer"],
        sections: sections
            .into_iter()
            .map(|(name, binds)| Section { name, binds })
            .collect(),
    }
}

fn action_id(action: &str) -> String {
    action.to_lowercase().replace(' ', "_")
}

fn read_overrides() -> std::collections::HashMap<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    let path = std::path::PathBuf::from(home).join(".config/herdr/config.toml");
    let Ok(content) = fs::read_to_string(path) else {
        return Default::default();
    };
    parse_keys_section(&content)
}

/// Very small TOML `[keys]` table reader — just `key = "value"` lines, no
/// full parser needed for this one section.
fn parse_keys_section(content: &str) -> std::collections::HashMap<String, String> {
    let mut in_keys = false;
    let mut map = std::collections::HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_keys = line == "[keys]";
            continue;
        }
        if !in_keys {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim().to_string();
            // Values are either a plain string ("prefix+q") or a TOML array
            // of alternates (["prefix+h", "alt+enter"]) — quoted substrings
            // cover both without a real TOML parser.
            let quoted: Vec<&str> = v.trim().split('"').skip(1).step_by(2).collect();
            if quoted.is_empty() {
                continue;
            }
            let val = quoted
                .iter()
                .map(|s| s.replace("prefix", "Prefix").replace('+', " + "))
                .collect::<Vec<_>>()
                .join(" / ");
            map.insert(key, val);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_string_value() {
        let map = parse_keys_section("[keys]\nprefix = \"ctrl+space\"\n");
        assert_eq!(map.get("prefix").unwrap(), "ctrl + space");
    }

    #[test]
    fn parses_array_of_alternates() {
        let map = parse_keys_section(
            "[keys]\nsplit_vertical = [\"prefix+v\", \"alt+shift+enter\"]\n",
        );
        assert_eq!(
            map.get("split_vertical").unwrap(),
            "Prefix + v / alt + shift + enter"
        );
    }

    #[test]
    fn ignores_keys_outside_the_keys_section() {
        let map = parse_keys_section("[other]\nprefix = \"ctrl+space\"\n");
        assert!(map.is_empty());
    }
}
