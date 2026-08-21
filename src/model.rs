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

impl Tab {
    pub fn flat(&self) -> Vec<(&str, &Bind)> {
        self.sections
            .iter()
            .flat_map(|s| s.binds.iter().map(move |b| (s.name.as_str(), b)))
            .collect()
    }

    pub fn filtered(&self, search: &str) -> Vec<(&str, &Bind)> {
        let needle = search.to_lowercase();
        self.flat()
            .into_iter()
            .filter(|(section, bind)| {
                needle.is_empty()
                    || bind.keys.to_lowercase().contains(&needle)
                    || bind.action.to_lowercase().contains(&needle)
                    || section.to_lowercase().contains(&needle)
            })
            .collect()
    }
}
