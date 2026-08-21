use crate::model::Tab;
use std::process::Command;

const TERMINAL_CLASSES: &[&str] = &["ghostty", "kitty", "com.mitchellh.ghostty"];

/// Best-effort guess at which tab should be selected on launch: reads the
/// currently focused window's class via `hyprctl activewindow -j`, matches it
/// against each tab's `window_class` list, and for terminal classes (which
/// don't map to a single app) walks the process tree under that window's pid
/// looking for `nvim` or `herdr` so we can tell those two apart.
pub fn guess_app(tabs: &[Tab]) -> Option<String> {
    // Launcher scripts (scratchkeys) capture the previously-focused window
    // before opening kb's own window and hand it down via env, since by the
    // time we run, kb's own terminal is what's actually focused.
    let (class, pid) = match (
        std::env::var("KB_FOCUS_CLASS").ok().filter(|s| !s.is_empty()),
        std::env::var("KB_FOCUS_PID").ok().and_then(|s| s.parse::<i64>().ok()),
    ) {
        (Some(class), pid) => (class.to_lowercase(), pid),
        _ => {
            let json = run(&["hyprctl", "activewindow", "-j"])?;
            let value: serde_json::Value = serde_json::from_str(&json).ok()?;
            let class = value.get("class")?.as_str()?.to_lowercase();
            let pid = value.get("pid").and_then(|v| v.as_i64());
            (class, pid)
        }
    };

    if let Some(tab) = tabs
        .iter()
        .find(|t| t.window_class.iter().any(|c| class.contains(c)))
    {
        return Some(tab.app.clone());
    }

    if TERMINAL_CLASSES.iter().any(|c| class.contains(c)) {
        if let Some(pid) = pid {
            let tree = process_tree_names(pid);
            if tree.iter().any(|n| n == "nvim") {
                return Some("Neovim".into());
            }
            if tree.iter().any(|n| n == "herdr") {
                return Some("Herdr".into());
            }
            if tree.iter().any(|n| n == "spotify_player") {
                return Some("Spotify".into());
            }
            if tree.iter().any(|n| n == "lazygit") {
                return Some("Lazygit".into());
            }
            if tree.iter().any(|n| n == "yazi") {
                return Some("Yazi".into());
            }
            if tree.iter().any(|n| n == "glow") {
                return Some("Glow".into());
            }
            if tree.iter().any(|n| n == "tuicr") {
                return Some("Tuicr".into());
            }
        }
    }

    None
}

fn process_tree_names(root_pid: i64) -> Vec<String> {
    let mut names = Vec::new();
    let mut queue = vec![root_pid];
    while let Some(pid) = queue.pop() {
        let Some(out) = run(&["ps", "--ppid", &pid.to_string(), "-o", "pid=,comm="]) else {
            continue;
        };
        for line in out.lines() {
            let mut parts = line.split_whitespace();
            let Some(child_pid) = parts.next().and_then(|p| p.parse::<i64>().ok()) else {
                continue;
            };
            let comm = parts.collect::<Vec<_>>().join(" ");
            names.push(comm);
            queue.push(child_pid);
        }
    }
    names
}

fn run(argv: &[&str]) -> Option<String> {
    let out = Command::new(argv[0]).args(&argv[1..]).output().ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}
