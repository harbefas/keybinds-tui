use crate::model::{Bind, Section, Tab};
use std::fs;

/// herdr has no `dump-keybindings` command and its defaults are compiled into
/// the binary (src/ui/keybind_help.rs), so the base table below is hand-copied
/// from that source once. What we *can* read live is ~/.config/herdr/config.toml's
/// [keys] table, which overrides individual defaults — those are merged in.
const DEFAULTS: &[(&str, &str, &str)] = &[
    ("Workspaces", "Prefix + Down", "Próximo workspace"),
    ("Workspaces", "Prefix + Up", "Workspace anterior"),
    ("Workspaces", "Prefix + W", "Workspace picker"),
    ("Workspaces", "Prefix + Shift + N", "Novo workspace"),
    ("Workspaces", "Prefix + Shift + W", "Renomear workspace"),
    ("Workspaces", "Prefix + Shift + D", "Fechar workspace"),
    ("Tabs", "Prefix + Right", "Próxima tab"),
    ("Tabs", "Prefix + Left", "Tab anterior"),
    ("Tabs", "Prefix + C", "Nova tab"),
    ("Tabs", "Prefix + Shift + T", "Renomear tab"),
    ("Tabs", "Prefix + Shift + X", "Fechar tab"),
    ("Tabs", "Prefix + 1..9", "Ir para tab N"),
    ("Panes", "Prefix + V", "Split vertical"),
    ("Panes", "Prefix + -", "Split horizontal"),
    ("Panes", "Prefix + X", "Fechar pane"),
    ("Panes", "Prefix + Z", "Zoom (fullscreen) pane"),
    ("Panes", "Prefix + H/J/K/L", "Foco pane esq/baixo/cima/dir"),
    ("Panes", "Prefix + Tab", "Cyclar panes"),
    ("Panes", "Prefix + R", "Modo resize"),
    ("Misc", "Prefix + ?", "Help"),
    ("Misc", "Prefix + B", "Toggle sidebar"),
    ("Misc", "Prefix + G", "Goto (busca)"),
    ("Misc", "Prefix + E", "Editar scrollback"),
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
        sections: sections
            .into_iter()
            .map(|(name, binds)| Section { name, binds })
            .collect(),
    }
}

fn action_id(action: &str) -> String {
    action.to_lowercase().replace(' ', "_")
}

/// Very small TOML `[keys]` table reader — just `key = "value"` lines, no
/// full parser needed for this one section.
fn read_overrides() -> std::collections::HashMap<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    let path = std::path::PathBuf::from(home).join(".config/herdr/config.toml");
    let Ok(content) = fs::read_to_string(path) else {
        return Default::default();
    };

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
            let val = v.trim().trim_matches('"').replace("prefix", "Prefix").replace('+', " + ");
            map.insert(key, val);
        }
    }
    map
}
