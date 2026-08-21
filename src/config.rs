use crate::model::{Bind, Section, Tab};
use serde::Deserialize;

/// User-defined tabs, merged in after the built-in sources so people can add
/// their own apps without forking. Format:
///
/// ```toml
/// [[tab]]
/// app = "MyApp"
/// window_class = ["myapp"]
///
/// [[tab.section]]
/// name = "General"
///
/// [[tab.section.bind]]
/// keys = "Ctrl+X"
/// action = "Do something"
/// ```
#[derive(Deserialize)]
struct FileConfig {
    #[serde(default, rename = "tab")]
    tabs: Vec<FileTab>,
}

#[derive(Deserialize)]
struct FileTab {
    app: String,
    #[serde(default)]
    window_class: Vec<String>,
    #[serde(default, rename = "section")]
    sections: Vec<FileSection>,
}

#[derive(Deserialize)]
struct FileSection {
    name: String,
    #[serde(default, rename = "bind")]
    binds: Vec<FileBind>,
}

#[derive(Deserialize)]
struct FileBind {
    keys: String,
    action: String,
}

pub fn load_user_tabs() -> Vec<Tab> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    let path = std::path::PathBuf::from(home).join(".config/kb/tabs.toml");
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(cfg) = toml::from_str::<FileConfig>(&content) else {
        return Vec::new();
    };

    cfg.tabs
        .into_iter()
        .map(|t| Tab {
            app: t.app,
            // Tab::window_class is &'static — user config is read once at
            // startup and lives for the process, so leaking it is fine.
            window_class: Box::leak(
                t.window_class
                    .into_iter()
                    .map(|s| -> &'static str { s.leak() })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            sections: t
                .sections
                .into_iter()
                .map(|s| Section {
                    name: s.name,
                    binds: s
                        .binds
                        .into_iter()
                        .map(|b| Bind {
                            keys: b.keys,
                            action: b.action,
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect()
}
