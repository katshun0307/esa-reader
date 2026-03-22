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
        use md_tui::util::colors::{
            color_config, heading_colors, set_color_config, set_heading_colors,
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ── parse_hex_color ───────────────────────────────────────────────────────

    #[rstest]
    #[case("#000000", Color::Rgb(0, 0, 0))]
    #[case("#FFFFFF", Color::Rgb(255, 255, 255))]
    #[case("#ffffff", Color::Rgb(255, 255, 255))]
    #[case("#1A2B3C", Color::Rgb(0x1A, 0x2B, 0x3C))]
    #[case("#38BDF8", Color::Rgb(0x38, 0xBD, 0xF8))]
    fn test_parse_hex_color_valid(#[case] input: &str, #[case] expected: Color) {
        assert_eq!(parse_hex_color(input), Some(expected));
    }

    #[rstest]
    #[case("", "empty string")]
    #[case("FFFFFF", "missing # prefix")]
    #[case("#FFF", "3-digit shorthand not supported")]
    #[case("#GGGGGG", "invalid hex digits")]
    #[case("#12345", "five hex digits")]
    #[case("#1234567", "seven hex digits")]
    fn test_parse_hex_color_invalid(#[case] input: &str, #[case] _reason: &str) {
        assert_eq!(parse_hex_color(input), None, "expected None for: {input}");
    }

    // ── parse_or_default ──────────────────────────────────────────────────────

    #[rstest]
    fn test_parse_or_default_uses_value_when_valid() {
        let color = parse_or_default(Some("#FF0000"), Some("#00FF00"));
        assert_eq!(color, Color::Rgb(255, 0, 0));
    }

    #[rstest]
    fn test_parse_or_default_falls_back_to_fallback() {
        let color = parse_or_default(None, Some("#00FF00"));
        assert_eq!(color, Color::Rgb(0, 255, 0));
    }

    #[rstest]
    fn test_parse_or_default_returns_reset_when_both_none() {
        let color = parse_or_default(None, None);
        assert_eq!(color, Color::Reset);
    }

    #[rstest]
    fn test_parse_or_default_invalid_value_falls_back() {
        // Invalid primary → should use fallback
        let color = parse_or_default(Some("not-a-color"), Some("#0000FF"));
        assert_eq!(color, Color::Rgb(0, 0, 255));
    }

    // ── Theme::from_config ────────────────────────────────────────────────────

    #[rstest]
    fn test_theme_from_config_full() {
        let config = ThemeConfig {
            primary: Some("#FF0000".to_string()),
            muted: Some("#00FF00".to_string()),
            accent: Some("#0000FF".to_string()),
            error: Some("#FF00FF".to_string()),
            success: Some("#00FFFF".to_string()),
            warning: Some("#FFFF00".to_string()),
            link: Some("#FFFFFF".to_string()),
        };

        let theme = Theme::from_config(&config);
        assert_eq!(theme.primary, Color::Rgb(255, 0, 0));
        assert_eq!(theme.muted, Color::Rgb(0, 255, 0));
        assert_eq!(theme.accent, Color::Rgb(0, 0, 255));
        assert_eq!(theme.error, Color::Rgb(255, 0, 255));
        assert_eq!(theme.success, Color::Rgb(0, 255, 255));
        assert_eq!(theme.warning, Color::Rgb(255, 255, 0));
        assert_eq!(theme.link, Color::Rgb(255, 255, 255));
    }

    /// An empty config should fall through to the dark-theme defaults (which
    /// are defined as valid hex values), so no color should be `Reset`.
    #[rstest]
    fn test_theme_from_config_empty_falls_back_to_dark_defaults() {
        let empty = ThemeConfig {
            primary: None,
            muted: None,
            accent: None,
            error: None,
            success: None,
            warning: None,
            link: None,
        };

        let theme = Theme::from_config(&empty);
        // Dark theme primary is #E2E8F0 — verify it isn't Reset
        assert_ne!(theme.primary, Color::Reset);
        assert_ne!(theme.accent, Color::Reset);
    }
}
