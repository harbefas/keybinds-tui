use crate::model::Tab;

/// No local .tridactylrc — config lives in browser sync storage (`:set`
/// commands run inside LibreWolf), nothing to read from disk. Static table
/// hand-copied from the vault note until we wire a native-messaging reader.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navigation",
            &[
                ("j / k", "Scroll down / up"),
                ("h / l", "Scroll left / right"),
                ("gg / G", "Go to top / bottom"),
                ("H / L", "Back / Forward"),
                ("r", "Reload"),
                ("R", "Hard reload"),
            ],
        ),
        (
            "Links",
            &[
                ("f", "Hint mode — open in same tab"),
                ("F", "Hint mode — open in new tab"),
                ("gi", "Focus first input on the page"),
            ],
        ),
        (
            "Tabs",
            &[
                ("t", "New tab with URL"),
                ("d", "Close tab"),
                ("u", "Reopen closed tab"),
                ("J / K", "Previous / next tab"),
                ("b", "Buffer picker (search by tab)"),
                ("gt / gT", "Next / previous tab"),
            ],
        ),
        (
            "URL / Search",
            &[
                ("o", "Open URL in same tab"),
                ("O", "Open URL in new tab"),
                ("s", "Search in same tab"),
                ("S", "Search in new tab"),
                ("p / P", "Open clipboard as URL"),
            ],
        ),
        (
            "Misc",
            &[
                ("yy", "Copy page URL"),
                ("yt", "Duplicate tab"),
                ("zi / zo / zz", "Zoom in / out / reset"),
                (":help", "Full help"),
                (":tutor", "Interactive tutorial"),
                ("Escape", "Back to normal mode"),
            ],
        ),
    ];

    Tab::from_raw(
        "Tridactyl",
        &["librewolf", "firefox"],
        &["vim", "browser"],
        raw,
    )
}
