use crate::model::{Bind, Section, Tab};

/// No `keybinding:` override block in ~/.config/lazygit/config.yml — static
/// table hand-copied from lazygit's documented default keybindings.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Global",
            &[
                ("1..5", "Trocar de painel (Status/Files/Branches/Commits/Stash)"),
                ("Tab", "Próximo painel"),
                ("[ / ]", "Aba anterior / próxima dentro do painel"),
                ("+ / -", "Aumentar / diminuir painel"),
                ("P", "Push"),
                ("p", "Pull"),
                ("x", "Menu de opções do painel"),
                ("?", "Help (lista completa de shortcuts)"),
                ("q", "Sair"),
            ],
        ),
        (
            "Files",
            &[
                ("space", "Stage / unstage arquivo"),
                ("a", "Stage all"),
                ("c", "Commit"),
                ("C", "Commit (editor externo)"),
                ("d", "Discard changes"),
                ("e", "Editar arquivo"),
                ("i", "Ignore file"),
                ("D", "Reset menu"),
                ("Enter", "Ver diff em stage por hunk/linha"),
            ],
        ),
        (
            "Branches",
            &[
                ("space", "Checkout branch"),
                ("n", "Nova branch"),
                ("d", "Delete branch"),
                ("M", "Merge branch selecionada na atual"),
                ("r", "Rebase branch atual sobre a selecionada"),
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
                ("p", "Pick (durante rebase interativo)"),
                ("g", "Reset atual pra esse commit"),
                ("C", "Copiar commit (cherry-pick)"),
                ("V", "Colar commits copiados"),
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
