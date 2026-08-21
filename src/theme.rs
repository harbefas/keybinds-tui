use ratatui::style::Color;
use std::process::Command;

// mateCreations design system — Yerba Mate (dark) / Tererê (light).
// Source of truth: ~/code/personal/mateCreations/ui/tokens/primitives.json
// Auto-switch matches the spec: dark 18h-6h, light 6h-18h (DESIGN.md §1/§7).
pub struct Theme {
    pub bg: Color,
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub text_2: Color,
    pub text_3: Color,
    pub text_4: Color,
    /// `--accent-text`, not `--accent`: the fill color fails contrast as text
    /// in Tererê, so the design system splits fill from ink (DESIGN.md §2).
    pub accent_text: Color,
}

const YERBA_MATE: Theme = Theme {
    bg: Color::Rgb(0x28, 0x2d, 0x1c),
    surface: Color::Rgb(0x40, 0x48, 0x34),
    border: Color::Rgb(0x4f, 0x5b, 0x4a),
    text: Color::Rgb(0xdc, 0xe0, 0xd9),
    text_2: Color::Rgb(0xb8, 0xc0, 0xaf),
    text_3: Color::Rgb(0xa0, 0xab, 0x98),
    text_4: Color::Rgb(0x7a, 0x85, 0x73),
    accent_text: Color::Rgb(0xd4, 0xa0, 0x33),
};

const TERERE: Theme = Theme {
    bg: Color::Rgb(0xfb, 0xf1, 0xc7),
    surface: Color::Rgb(0xf5, 0xea, 0xc0),
    border: Color::Rgb(0xdd, 0xd2, 0xa0),
    text: Color::Rgb(0x3c, 0x38, 0x36),
    text_2: Color::Rgb(0x50, 0x49, 0x45),
    text_3: Color::Rgb(0x66, 0x5c, 0x54),
    text_4: Color::Rgb(0x7c, 0x6f, 0x64),
    accent_text: Color::Rgb(0x8f, 0x4f, 0x00),
};

pub fn current() -> &'static Theme {
    if is_daytime() {
        &TERERE
    } else {
        &YERBA_MATE
    }
}

fn is_daytime() -> bool {
    let hour = Command::new("date")
        .arg("+%H")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(12);
    (6..18).contains(&hour)
}
