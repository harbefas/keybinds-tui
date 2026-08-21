#[derive(Clone)]
pub struct Bind {
    pub keys: String,
    pub action: String,
}

#[derive(Clone)]
pub struct Section {
    pub name: String,
    pub binds: Vec<Bind>,
}

#[derive(Clone)]
pub struct Tab {
    pub app: String,
    pub window_class: &'static [&'static str],
    pub sections: Vec<Section>,
}

/// Section name -> list of (keys, action) pairs, the shape every static
/// source table is hand-written in.
pub type RawSections<'a> = &'a [(&'a str, &'a [(&'a str, &'a str)])];

impl Tab {
    /// Builds a `Tab` from a static `RawSections` table — the common shape
    /// shared by every source with no live config to parse.
    pub fn from_raw(app: &str, window_class: &'static [&'static str], raw: RawSections) -> Tab {
        Tab {
            app: app.to_string(),
            window_class,
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

    pub fn flat(&self) -> Vec<(&str, &Bind)> {
        self.sections
            .iter()
            .flat_map(|s| s.binds.iter().map(move |b| (s.name.as_str(), b)))
            .collect()
    }

    /// Every whitespace-separated word in `search` must appear somewhere in
    /// the row (section, keys, or action), in any order — so "tab next" and
    /// "next tab" both match a row whose action is "Next tab".
    pub fn filtered(&self, search: &str) -> Vec<(&str, &Bind)> {
        let words: Vec<String> = search.to_lowercase().split_whitespace().map(String::from).collect();
        self.flat()
            .into_iter()
            .filter(|(section, bind)| {
                let haystack = format!(
                    "{} {} {}",
                    section.to_lowercase(),
                    bind.keys.to_lowercase(),
                    bind.action.to_lowercase()
                );
                words.iter().all(|w| haystack.contains(w.as_str()))
            })
            .collect()
    }
}
