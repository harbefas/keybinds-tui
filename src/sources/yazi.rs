use crate::model::{Bind, Section, Tab};

/// No ~/.config/yazi/keymap.toml override — defaults are compiled into the
/// binary. Static table hand-copied from yazi's documented default keymap.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navigation",
            &[
                ("j / k", "Down / up"),
                ("h / l", "Leave / enter directory"),
                ("g g / G", "Top / bottom of list"),
                ("Ctrl+u / Ctrl+d", "Half page up / down"),
                ("Tab", "Switch between panels"),
                ("Enter", "Open file / enter directory"),
            ],
        ),
        (
            "Selection",
            &[
                ("space", "Select/deselect item"),
                ("v", "Visual mode (select)"),
                ("V", "Visual mode (deselect)"),
                ("Ctrl+a", "Select all"),
                ("Ctrl+r", "Invert selection"),
                ("Esc", "Cancel selection / visual mode"),
            ],
        ),
        (
            "Files",
            &[
                ("y", "Copy (yank)"),
                ("x", "Cut"),
                ("p", "Paste"),
                ("P", "Paste without overwrite"),
                ("d", "Move to trash"),
                ("D", "Delete permanently"),
                ("a", "Create file/folder"),
                ("r", "Rename"),
                (":", "Create (shell command)"),
            ],
        ),
        (
            "Search and quick navigation",
            &[
                ("/", "Search (fzf-like filter)"),
                ("f", "Find (incremental search)"),
                ("z", "Zoxide jump"),
                ("Z", "Fzf jump"),
                ("`", "Go to directory (goto)"),
                ("[ / ]", "Jump to previous / next dir in history"),
            ],
        ),
        (
            "Misc",
            &[
                ("~", "Open help (full shortcut list)"),
                ("s", "Shell (command in current directory)"),
                (".", "Toggle show hidden"),
                ("T", "Toggle tab preview"),
                ("q", "Quit"),
            ],
        ),
    ];

    Tab {
        app: "Yazi".into(),
        window_class: &["yazi"],
        sections: raw
            .iter()
            .map(|(name, binds)| Section {
                name: name.to_string(),
                binds: binds
                    .iter()
                    .map(|(k, a)| Bind {
                        keys: k.to_string(),
                        action: a.to_string(),
                    })
                    .collect(),
            })
            .collect(),
    }
}
