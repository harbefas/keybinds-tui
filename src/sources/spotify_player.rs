use crate::model::{Bind, Section, Tab};

/// No local keymap override under ~/.config/spotify-player (only app.toml,
/// no keymap.toml) — static table hand-copied from spotify_player's
/// documented default keymaps.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Playback",
            &[
                ("space", "Play / pause"),
                ("n / p", "Próxima / anterior faixa"),
                ("< / >", "Retroceder / avançar 5s"),
                ("- / +", "Volume down / up"),
                (". ", "Toggle repeat"),
                (",", "Toggle shuffle"),
                ("z", "Toggle like"),
            ],
        ),
        (
            "Navegação",
            &[
                ("j / k", "Baixo / cima"),
                ("h / l", "Esquerda / direita"),
                ("g / G", "Topo / fim"),
                ("Tab / Shift+Tab", "Próxima / página anterior"),
                ("Enter", "Selecionar"),
                ("Escape", "Voltar / fechar popup"),
            ],
        ),
        (
            "Busca e contexto",
            &[
                ("/", "Buscar"),
                ("s", "Abrir search page"),
                ("c", "Abrir contexto (playlist/álbum/artista) atual"),
                ("C", "Menu de contexto (browse)"),
                ("q", "Adicionar à queue"),
                ("Q", "Abrir queue"),
            ],
        ),
        (
            "Devices e Misc",
            &[
                ("D", "Selecionar device"),
                ("?", "Help popup (lista completa de shortcuts)"),
                ("Ctrl+q", "Sair"),
            ],
        ),
    ];

    Tab {
        app: "Spotify".into(),
        window_class: &["spotify_player", "spotify-player"],
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
