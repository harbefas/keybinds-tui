use crate::model::{Bind, Section, Tab};

/// Static table from tuicr's documented vim-style keybindings (docs at
/// tuicr.dev) — no local config file to source live from yet.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navegação",
            &[
                ("j / k", "Linha baixo / cima"),
                ("h / l", "Colapsar / expandir hunk ou arquivo"),
                ("g g / G", "Topo / fim do diff"),
                ("Ctrl+d / Ctrl+u", "Meia página baixo / cima"),
                ("Tab / Shift+Tab", "Próximo / arquivo anterior"),
                ("]c / [c", "Próxima / hunk anterior"),
            ],
        ),
        (
            "Comentários",
            &[
                ("c", "Comentar linha atual"),
                ("v", "Modo visual — selecionar range de linhas"),
                ("C", "Comentar arquivo inteiro"),
                ("R", "Comentário geral da review"),
                ("e", "Editar comentário existente"),
                ("d", "Deletar comentário"),
            ],
        ),
        (
            "Review e submissão",
            &[
                ("s", "Submeter review (push pro forge)"),
                ("y", "Copiar review em markdown"),
                ("Enter", "Aprovar / marcar hunk como revisado"),
                ("?", "Help (lista completa de shortcuts)"),
                ("q", "Sair"),
            ],
        ),
    ];

    Tab {
        app: "Tuicr".into(),
        window_class: &["tuicr"],
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
