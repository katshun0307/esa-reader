use ratatui::style::Color;

use crate::domains::ThemeConfig;

#[derive(Clone, Debug)]
pub struct Theme {
    pub primary: Color,
    pub muted: Color,
    pub accent: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub link: Color,
}

impl Theme {
    pub fn from_config(config: &ThemeConfig) -> Self {
        let defaults = ThemeConfig::default();
        Self {
            primary: parse_or_default(config.primary.as_deref(), defaults.primary.as_deref()),
            muted: parse_or_default(config.muted.as_deref(), defaults.muted.as_deref()),
            accent: parse_or_default(config.accent.as_deref(), defaults.accent.as_deref()),
            error: parse_or_default(config.error.as_deref(), defaults.error.as_deref()),
            success: parse_or_default(config.success.as_deref(), defaults.success.as_deref()),
            warning: parse_or_default(config.warning.as_deref(), defaults.warning.as_deref()),
            link: parse_or_default(config.link.as_deref(), defaults.link.as_deref()),
        }
    }
}

fn parse_or_default(value: Option<&str>, fallback: Option<&str>) -> Color {
    if let Some(value) = value.and_then(parse_hex_color) {
        return value;
    }
    if let Some(value) = fallback.and_then(parse_hex_color) {
        return value;
    }
    Color::Reset
}

fn parse_hex_color(value: &str) -> Option<Color> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}
