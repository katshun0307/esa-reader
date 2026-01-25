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

    pub fn apply_to_md_tui(&self) {
        use md_tui::util::colors::{color_config, heading_colors, set_color_config, set_heading_colors};

        let mut config = color_config();
        // Map minimal theme colors while preserving md-tui defaults for the rest.
        config.heading_fg_color = self.primary;
        config.bold_color = self.primary;
        config.bold_italic_color = self.primary;
        config.italic_color = self.muted;
        config.code_fg_color = self.accent;
        config.link_color = self.link;
        config.link_selected_fg_color = self.link;
        config.table_header_fg_color = self.accent;
        config.quote_important = self.error;
        config.quote_warning = self.warning;
        config.quote_tip = self.success;
        set_color_config(config);

        let mut headings = heading_colors();
        headings.level_2 = self.primary;
        headings.level_3 = self.accent;
        headings.level_4 = self.accent;
        headings.level_5 = self.muted;
        headings.level_6 = self.muted;
        set_heading_colors(headings);
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
