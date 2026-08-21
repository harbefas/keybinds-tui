use crate::model::{Bind, Section, Tab};

/// No local .tridactylrc — config lives in browser sync storage (`:set`
/// commands run inside LibreWolf), nothing to read from disk. Static table
/// hand-copied from the vault note until we wire a native-messaging reader.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navegação",
            &[
                ("j / k", "Scroll baixo / cima"),
                ("h / l", "Scroll esquerda / direita"),
                ("gg / G", "Ir para topo / fim"),
                ("H / L", "Voltar / Avançar"),
                ("r", "Reload"),
                ("R", "Hard reload"),
            ],
        ),
        (
            "Links",
            &[
                ("f", "Hint mode — abrir na mesma aba"),
                ("F", "Hint mode — abrir em nova aba"),
                ("gi", "Focar primeiro input da página"),
            ],
        ),
        (
            "Tabs",
            &[
                ("t", "Nova tab com URL"),
                ("d", "Fechar tab"),
                ("u", "Reabrir tab fechada"),
                ("J / K", "Tab anterior / próxima"),
                ("b", "Buffer picker (busca por tab)"),
                ("gt / gT", "Tab seguinte / anterior"),
            ],
        ),
        (
            "URL / Busca",
            &[
                ("o", "Abrir URL na mesma aba"),
                ("O", "Abrir URL em nova aba"),
                ("s", "Buscar na mesma aba"),
                ("S", "Buscar em nova aba"),
                ("p / P", "Abrir clipboard como URL"),
            ],
        ),
        (
            "Misc",
            &[
                ("yy", "Copiar URL da página"),
                ("yt", "Duplicar tab"),
                ("zi / zo / zz", "Zoom in / out / reset"),
                (":help", "Ajuda completa"),
                (":tutor", "Tutorial interativo"),
                ("Escape", "Voltar ao modo normal"),
            ],
        ),
    ];

    Tab {
        app: "Tridactyl".into(),
        window_class: &["librewolf", "firefox"],
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
