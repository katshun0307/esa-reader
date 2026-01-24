use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub workspaces: BTreeMap<String, WorkspaceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkspaceConfig {
    team_name: String,
    #[serde(default = "default_endpoint")]
    api_endpoint: String,
    token: String,
}

fn default_endpoint() -> String {
    "https://api.esa.io".to_string()
}
