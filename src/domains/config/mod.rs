use std::{collections::BTreeMap, sync::LazyLock};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub workspaces: BTreeMap<String, WorkspaceConfig>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub themes: BTreeMap<String, ThemeConfig>,
}

impl Config {
    pub fn current_workspace(&self) -> (String, WorkspaceConfig) {
        self.workspaces.clone().into_iter().nth(0).unwrap()
    }

    pub fn get_theme(&self, workspace_name: &str) -> ThemeConfig {
        if let Some(workspace) = self.workspaces.get(workspace_name) {
            if let Some(theme_name) = &workspace.theme {
                if let Some(theme) = self.themes.get(theme_name) {
                    return theme.clone();
                } else {
                    if theme_name == "dark" {
                        return THEME_CONFIG_DARK.clone();
                    } else if theme_name == "light" {
                        return THEME_CONFIG_LIGHT.clone();
                    }
                }
            }
        }
        ThemeConfig::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkspaceConfig {
    team_name: String,
    #[serde(default = "default_endpoint")]
    api_endpoint: String,
    token: String,
    pub post_views: BTreeMap<String, PostViewConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostViewConfig {
    pub title: String,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThemeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

static THEME_CONFIG_DARK: LazyLock<ThemeConfig> = LazyLock::new(|| ThemeConfig {
    primary: Some("#E2E8F0".to_string()),
    muted: Some("#94A3B8".to_string()),
    accent: Some("#38BDF8".to_string()),
    error: Some("#F87171".to_string()),
    success: Some("#34D399".to_string()),
    warning: Some("#FBBF24".to_string()),
    link: Some("#60A5FA".to_string()),
});

static THEME_CONFIG_LIGHT: LazyLock<ThemeConfig> = LazyLock::new(|| ThemeConfig {
    primary: Some("#0F172A".to_string()),
    muted: Some("#475569".to_string()),
    accent: Some("#0284C7".to_string()),
    error: Some("#DC2626".to_string()),
    success: Some("#059669".to_string()),
    warning: Some("#D97706".to_string()),
    link: Some("#2563EB".to_string()),
});

impl Default for ThemeConfig {
    // use dark
    fn default() -> Self {
        THEME_CONFIG_DARK.clone()
    }
}

impl WorkspaceConfig {
    pub fn team_name(&self) -> String {
        self.team_name.clone()
    }

    #[allow(dead_code)]
    pub fn api_endpoint(&self) -> String {
        self.api_endpoint.clone()
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }
}

fn default_endpoint() -> String {
    "https://api.esa.io".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use rstest::fixture;

    #[fixture]
    fn config() -> Config {
        Config {
            workspaces: BTreeMap::from([(
                "default".to_string(),
                WorkspaceConfig {
                    team_name: "my_team".to_string(),
                    api_endpoint: "https://api.esa.io".to_string(),
                    token: "my_token".to_string(),
                    post_views: BTreeMap::from([(
                        "all".to_string(),
                        PostViewConfig {
                            title: "All Posts".to_string(),
                            query: Some("sort:updated".to_string()),
                        },
                    )]),
                    theme: Some("dark".to_string()),
                },
            )]),
            themes: BTreeMap::from([
                ("dark".to_string(), THEME_CONFIG_DARK.clone()),
                ("light".to_string(), THEME_CONFIG_LIGHT.clone()),
            ]),
        }
    }

    #[rstest::rstest]
    fn test_serialize_config(config: Config) {
        let toml_str = toml::to_string(&config).unwrap();
        assert_snapshot!(toml_str);
    }
}
