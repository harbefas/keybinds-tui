use crate::model::Tab;

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

    Tab::from_raw("Glow", &["glow"], raw)
}
