use crate::model::Tab;

/// No local keymap override under ~/.config/spotify-player (only app.toml,
/// no keymap.toml) — static table hand-copied from spotify_player's
/// documented default keymaps.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Playback",
            &[
                ("space", "Play / pause"),
                ("n / p", "Next / previous track"),
                ("< / >", "Seek back / forward 5s"),
                ("- / +", "Volume down / up"),
                (". ", "Toggle repeat"),
                (",", "Toggle shuffle"),
                ("z", "Toggle like"),
            ],
        ),
        (
            "Navigation",
            &[
                ("j / k", "Down / up"),
                ("h / l", "Left / right"),
                ("g / G", "Top / bottom"),
                ("Tab / Shift+Tab", "Next / previous page"),
                ("Enter", "Select"),
                ("Escape", "Back / close popup"),
            ],
        ),
        (
            "Search and context",
            &[
                ("/", "Search"),
                ("s", "Open search page"),
                ("c", "Open current context (playlist/album/artist)"),
                ("C", "Context menu (browse)"),
                ("q", "Add to queue"),
                ("Q", "Open queue"),
            ],
        ),
        (
            "Devices and misc",
            &[
                ("D", "Select device"),
                ("?", "Help popup (full shortcut list)"),
                ("Ctrl+q", "Quit"),
            ],
        ),
    ];

    Tab::from_raw(
        "Spotify",
        &["spotify_player", "spotify-player"],
        &["music"],
        raw,
    )
}
