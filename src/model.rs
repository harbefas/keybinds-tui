use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Bind {
    pub keys: String,
    pub action: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub binds: Vec<Bind>,
}

#[derive(Clone)]
pub struct Tab {
    pub app: String,
    pub window_class: &'static [&'static str],
    /// Extra search terms that don't show up in the table but count as a
    /// match for the whole tab — e.g. Tridactyl's app name won't contain
    /// "vim" or "firefox", but people search those words for it.
    pub aliases: &'static [&'static str],
    pub sections: Vec<Section>,
}

/// Section name -> list of (keys, action) pairs, the shape every static
/// source table is hand-written in.
pub type RawSections<'a> = &'a [(&'a str, &'a [(&'a str, &'a str)])];

impl Tab {
    /// Builds a `Tab` from a static `RawSections` table — the common shape
    /// shared by every source with no live config to parse.
    pub fn from_raw(
        app: &str,
        window_class: &'static [&'static str],
        aliases: &'static [&'static str],
        raw: RawSections,
    ) -> Tab {
        Tab {
            app: app.to_string(),
            window_class,
            aliases,
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

    /// Every whitespace-separated word in `search` must appear — as a
    /// substring or, failing that, as an in-order subsequence (fuzzy, like
    /// fzf) — somewhere in the row (section, keys, action, or the tab's
    /// aliases). Results are ranked substring-matches-first.
    pub fn filtered(&self, search: &str) -> Vec<(&str, &Bind)> {
        let words: Vec<String> = search.to_lowercase().split_whitespace().map(String::from).collect();
        let alias_text = self.aliases.join(" ").to_lowercase();

        let mut scored: Vec<(i32, (&str, &Bind))> = self
            .flat()
            .into_iter()
            .filter_map(|(section, bind)| {
                let haystack = format!(
                    "{} {} {} {}",
                    section.to_lowercase(),
                    bind.keys.to_lowercase(),
                    bind.action.to_lowercase(),
                    alias_text
                );
                let mut score = 0;
                for w in &words {
                    if haystack.contains(w.as_str()) {
                        score += 2;
                    } else if is_subsequence(w, &haystack) {
                        score += 1;
                    } else {
                        return None;
                    }
                }
                Some((score, (section, bind)))
            })
            .collect();

        if !words.is_empty() {
            scored.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
        }
        scored.into_iter().map(|(_, row)| row).collect()
    }
}

/// True if every char of `needle` appears in `haystack` in order (not
/// necessarily contiguous) — a cheap fzf-style fuzzy match.
fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut chars = haystack.chars();
    needle.chars().all(|c| chars.any(|h| h == c))
}

/// If `search` starts with `@token`, splits it into the tab-selector token
/// and the remaining filter text (e.g. "@vim scroll" -> (Some("vim"),
/// "scroll")). Otherwise returns (None, search) unchanged.
pub fn split_at_tab(search: &str) -> (Option<&str>, &str) {
    let Some(rest) = search.strip_prefix('@') else {
        return (None, search);
    };
    let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let token = &rest[..end];
    if token.is_empty() {
        return (None, search);
    }
    (Some(token), rest[end..].trim_start())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tab() -> Tab {
        Tab::from_raw(
            "Tridactyl",
            &[],
            &["vim", "firefox"],
            &[("Tabs", &[("gt", "Next tab"), ("gT", "Previous tab")])],
        )
    }

    #[test]
    fn matches_multi_word_any_order() {
        let t = tab();
        assert_eq!(t.filtered("tab next").len(), 1);
        assert_eq!(t.filtered("next tab").len(), 1);
    }

    #[test]
    fn matches_via_alias() {
        let t = tab();
        assert_eq!(t.filtered("firefox next").len(), 1);
        assert_eq!(t.filtered("firefox drop").len(), 0);
    }

    #[test]
    fn fuzzy_subsequence_matches() {
        let t = tab();
        // "nxt" is a subsequence of "next" but not a substring.
        assert_eq!(t.filtered("nxt").len(), 1);
    }

    #[test]
    fn split_at_tab_extracts_token_and_remainder() {
        assert_eq!(split_at_tab("@vim scroll"), (Some("vim"), "scroll"));
        assert_eq!(split_at_tab("@vim"), (Some("vim"), ""));
        assert_eq!(split_at_tab("scroll"), (None, "scroll"));
        assert_eq!(split_at_tab("@"), (None, "@"));
    }
}
