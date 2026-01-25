use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub workspaces: BTreeMap<String, WorkspaceConfig>,
}

impl Config {
    pub fn current_workspace(&self) -> (String, WorkspaceConfig) {
        self.workspaces.clone().into_iter().nth(0).unwrap()
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostViewConfig {
    pub title: String,
    pub query: Option<String>,
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
                },
            )]),
        }
    }

    #[rstest::rstest]
    fn test_serialize_config(config: Config) {
        let toml_str = toml::to_string(&config).unwrap();
        assert_snapshot!(toml_str);
    }
}
