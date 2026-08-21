use crate::model::{Bind, Section, Tab};

/// No `keybinding:` override block in ~/.config/lazygit/config.yml — static
/// table hand-copied from lazygit's documented default keybindings.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Global",
            &[
                ("1..5", "Switch panel (Status/Files/Branches/Commits/Stash)"),
                ("Tab", "Next panel"),
                ("[ / ]", "Previous / next tab within panel"),
                ("+ / -", "Increase / decrease panel"),
                ("P", "Push"),
                ("p", "Pull"),
                ("x", "Panel options menu"),
                ("?", "Help (full shortcut list)"),
                ("q", "Quit"),
            ],
        ),
        (
            "Files",
            &[
                ("space", "Stage / unstage file"),
                ("a", "Stage all"),
                ("c", "Commit"),
                ("C", "Commit (external editor)"),
                ("d", "Discard changes"),
                ("e", "Edit file"),
                ("i", "Ignore file"),
                ("D", "Reset menu"),
                ("Enter", "View staged diff by hunk/line"),
            ],
        ),
        (
            "Branches",
            &[
                ("space", "Checkout branch"),
                ("n", "New branch"),
                ("d", "Delete branch"),
                ("M", "Merge selected branch into current"),
                ("r", "Rebase current branch onto selected"),
                ("R", "Rename branch"),
            ],
        ),
        (
            "Commits",
            &[
                ("space", "Checkout commit"),
                ("s", "Squash down"),
                ("f", "Fixup commit"),
                ("r", "Reword"),
                ("d", "Drop commit"),
                ("p", "Pick (during interactive rebase)"),
                ("g", "Reset current branch to this commit"),
                ("C", "Copy commit (cherry-pick)"),
                ("V", "Paste copied commits"),
            ],
        ),
        (
            "Stash",
            &[
                ("space", "Apply stash"),
                ("g", "Pop stash"),
                ("d", "Drop stash"),
            ],
        ),
    ];

    Tab {
        app: "Lazygit".into(),
        window_class: &["lazygit"],
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
