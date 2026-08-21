use crate::model::Tab;

/// Static table from tuicr's documented vim-style keybindings (docs at
/// tuicr.dev) — no local config file to source live from yet.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navigation",
            &[
                ("j / k", "Line down / up"),
                ("h / l", "Collapse / expand hunk or file"),
                ("g g / G", "Top / bottom of diff"),
                ("Ctrl+d / Ctrl+u", "Half page down / up"),
                ("Tab / Shift+Tab", "Next / previous file"),
                ("]c / [c", "Next / previous hunk"),
            ],
        ),
        (
            "Comments",
            &[
                ("c", "Comment on current line"),
                ("v", "Visual mode — select line range"),
                ("C", "Comment on whole file"),
                ("R", "General review comment"),
                ("e", "Edit existing comment"),
                ("d", "Delete comment"),
            ],
        ),
        (
            "Review and submission",
            &[
                ("s", "Submit review (push to forge)"),
                ("y", "Copy review as markdown"),
                ("Enter", "Approve / mark hunk as reviewed"),
                ("?", "Help (full shortcut list)"),
                ("q", "Quit"),
            ],
        ),
    ];

    Tab::from_raw("Tuicr", &["tuicr"], &["review", "pr", "code review"], raw)
}
