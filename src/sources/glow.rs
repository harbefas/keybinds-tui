use crate::model::{Bind, Section, Tab};

/// glow's TUI mode has no configurable keymap in glow.yml — static table
/// hand-copied from its documented default keybindings.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navigation (file finder)",
            &[
                ("j / k", "Down / up"),
                ("g g / G", "Top / bottom"),
                ("/", "Filter files"),
                ("Enter", "Open file"),
                ("q", "Quit"),
            ],
        ),
        (
            "Reading (pager)",
            &[
                ("j / k", "Scroll line down / up"),
                ("d / u", "Half page down / up"),
                ("g / G", "Top / bottom of document"),
                ("/", "Search in document"),
                ("n / N", "Next / previous match"),
                ("esc / q", "Back to file finder"),
                ("?", "Help"),
            ],
        ),
    ];

    Tab {
        app: "Glow".into(),
        window_class: &["glow"],
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
