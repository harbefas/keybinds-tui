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
            sections,
        },
        Err(_) => Tab {
            app: "Neovim".into(),
            window_class: &[],
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

fn dump() -> anyhow::Result<Vec<Section>> {
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

    Ok(by_mode
        .into_iter()
        .map(|(name, binds)| Section { name, binds })
        .collect())
}
