use crate::model::{Bind, Section, Tab};

/// No ~/.config/yazi/keymap.toml override — defaults are compiled into the
/// binary. Static table hand-copied from yazi's documented default keymap.
pub fn load() -> Tab {
    let raw: &[(&str, &[(&str, &str)])] = &[
        (
            "Navegação",
            &[
                ("j / k", "Baixo / cima"),
                ("h / l", "Sair / entrar no diretório"),
                ("g g / G", "Topo / fim da lista"),
                ("Ctrl+u / Ctrl+d", "Meia página cima / baixo"),
                ("Tab", "Alternar entre painéis"),
                ("Enter", "Abrir arquivo / entrar em diretório"),
            ],
        ),
        (
            "Seleção",
            &[
                ("space", "Selecionar/deselecionar item"),
                ("v", "Modo visual (seleção)"),
                ("V", "Modo visual (deseleção)"),
                ("Ctrl+a", "Selecionar tudo"),
                ("Ctrl+r", "Inverter seleção"),
                ("Esc", "Cancelar seleção / modo visual"),
            ],
        ),
        (
            "Arquivos",
            &[
                ("y", "Copiar (yank)"),
                ("x", "Recortar (cut)"),
                ("p", "Colar"),
                ("P", "Colar sem sobrescrever"),
                ("d", "Mover pra lixeira"),
                ("D", "Deletar permanentemente"),
                ("a", "Criar arquivo/pasta"),
                ("r", "Renomear"),
                (":", "Criar (shell command)"),
            ],
        ),
        (
            "Busca e navegação rápida",
            &[
                ("/", "Buscar (fzf-like filter)"),
                ("f", "Find (busca incremental)"),
                ("z", "Zoxide jump"),
                ("Z", "Fzf jump"),
                ("`", "Ir para diretório (goto)"),
                ("[ / ]", "Pular pro dir anterior / próximo no histórico"),
            ],
        ),
        (
            "Misc",
            &[
                ("~", "Abrir help (lista completa de shortcuts)"),
                ("s", "Shell (comando no diretório atual)"),
                (".", "Toggle mostrar ocultos"),
                ("T", "Toggle preview de tab"),
                ("q", "Sair"),
            ],
        ),
    ];

    Tab {
        app: "Yazi".into(),
        window_class: &["yazi"],
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
