use crate::model::{Bind, Section, Tab};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct RawMap {
    lhs: String,
    #[serde(default)]
    rhs: String,
    #[serde(default)]
    desc: Option<String>,
}

const DUMP_LUA: &str = r#"
local out = {}
for _, mode in ipairs({"n", "i", "v", "x"}) do
  for _, m in ipairs(vim.api.nvim_get_keymap(mode)) do
    table.insert(out, {mode = mode, lhs = m.lhs, rhs = m.rhs or "", desc = m.desc})
  end
end
local f = io.open(os.getenv("KB_NVIM_DUMP"), "w")
f:write(vim.json.encode(out))
f:close()
vim.cmd("qa!")
"#;

/// Placeholder shown while `spawn`'s background thread is still dumping.
pub fn loading_tab() -> Tab {
    Tab {
        app: "Neovim".into(),
        window_class: &[],
        aliases: &["vim", "editor"],
        sections: vec![],
    }
}

/// Runs the (slow, up to ~4s) headless dump on a background thread so the UI
/// can render immediately; the result arrives on the returned channel.
pub fn spawn() -> std::sync::mpsc::Receiver<Tab> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(load());
    });
    rx
}

/// Headless-dumps nvim's live keymap table (`nvim_get_keymap`) via a throwaway
/// instance. Only picks up maps set at startup — plugins that lazy-load on an
/// event/filetype won't show unless nvim already touched that trigger.
fn load() -> Tab {
    match dump() {
        Ok(sections) => Tab {
            app: "Neovim".into(),
            window_class: &[],
            aliases: &["vim", "editor"],
            sections,
        },
        Err(_) => Tab {
            app: "Neovim".into(),
            window_class: &[],
            aliases: &["vim", "editor"],
            sections: vec![Section {
                name: "error".into(),
                binds: vec![Bind {
                    keys: "-".into(),
                    action: "failed to run `nvim --headless` (binary in PATH?)".into(),
                }],
            }],
        },
    }
}

/// How long a cached dump stays valid. Neovim's own keymaps rarely change
/// between kb invocations, so caching turns the ~4s headless startup into an
/// instant load for anything short of a config edit + reopen within the TTL.
const CACHE_TTL_SECS: u64 = 300;

fn cache_path() -> std::path::PathBuf {
    std::env::temp_dir().join("kb-nvim-cache.json")
}

fn read_cache() -> Option<Vec<Section>> {
    let path = cache_path();
    let modified = std::fs::metadata(&path).ok()?.modified().ok()?;
    let age = std::time::SystemTime::now().duration_since(modified).ok()?;
    if age.as_secs() > CACHE_TTL_SECS {
        return None;
    }
    serde_json::from_str(&std::fs::read_to_string(&path).ok()?).ok()
}

fn write_cache(sections: &[Section]) {
    if let Ok(json) = serde_json::to_string(sections) {
        let _ = std::fs::write(cache_path(), json);
    }
}

fn dump() -> anyhow::Result<Vec<Section>> {
    if let Some(cached) = read_cache() {
        return Ok(cached);
    }

    let tmp = std::env::temp_dir().join(format!("kb-nvim-dump-{}.json", std::process::id()));
    // `timeout` guards against lazy.nvim plugins that hang headless startup
    // (update checks, notifications waiting on a UI that never attaches).
    let status = Command::new("timeout")
        .arg("4")
        .arg("nvim")
        .arg("--headless")
        .arg("-c")
        .arg(format!("lua {}", DUMP_LUA.replace('\n', " ")))
        .env("KB_NVIM_DUMP", &tmp)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;
    anyhow::ensure!(tmp.exists(), "nvim dump timed out or produced nothing");
    let _ = status;

    let raw = std::fs::read_to_string(&tmp)?;
    let _ = std::fs::remove_file(&tmp);
    let entries: Vec<serde_json::Value> = serde_json::from_str(&raw)?;

    let mut by_mode: std::collections::BTreeMap<String, Vec<Bind>> = Default::default();
    for e in entries {
        let mode = e.get("mode").and_then(|v| v.as_str()).unwrap_or("n");
        let raw_map: RawMap = serde_json::from_value(e.clone())?;
        if raw_map.lhs.trim().is_empty() {
            continue;
        }
        let action = raw_map
            .desc
            .filter(|d| !d.is_empty())
            .unwrap_or(raw_map.rhs);
        let mode_name = match mode {
            "n" => "Normal",
            "i" => "Insert",
            "v" => "Visual",
            "x" => "Visual (block)",
            other => other,
        };
        by_mode
            .entry(mode_name.to_string())
            .or_default()
            .push(Bind {
                keys: raw_map.lhs,
                action: if action.is_empty() { "<cmd lua>".into() } else { action },
            });
    }

    let sections: Vec<Section> = by_mode
        .into_iter()
        .map(|(name, binds)| Section { name, binds })
        .collect();
    write_cache(&sections);
    Ok(sections)
}
