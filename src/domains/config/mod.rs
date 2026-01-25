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
