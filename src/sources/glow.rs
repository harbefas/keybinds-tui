use crate::model::{Bind, Section, Tab};

/// glow's TUI mode has no configurable keymap in glow.yml — static table
/// hand-copied from its documented default keybindings.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navegação (file finder)",
            &[
                ("j / k", "Baixo / cima"),
                ("g g / G", "Topo / fim"),
                ("/", "Filtrar arquivos"),
                ("Enter", "Abrir arquivo"),
                ("q", "Sair"),
            ],
        ),
        (
            "Leitura (pager)",
            &[
                ("j / k", "Scroll linha baixo / cima"),
                ("d / u", "Meia página baixo / cima"),
                ("g / G", "Topo / fim do documento"),
                ("/", "Buscar no documento"),
                ("n / N", "Próxima / anterior ocorrência"),
                ("esc / q", "Voltar ao file finder"),
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
