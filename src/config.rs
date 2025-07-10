use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub verify: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub script: String,
    pub network: String,
    pub setup_command: String,
    pub repo: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvConfig {
    pub vars: HashMap<String, String>,
    pub load_files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentConfig {
    pub project: ProjectConfig,
    pub env: EnvConfig,
    pub networks: HashMap<String, NetworkConfig>,
}

impl DeploymentConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read configuration file")?;

        let config: DeploymentConfig =
            toml::from_str(&content).context("Failed to parse TOML configuration")?;

        Ok(config)
    }

    pub fn get_network(&self, network_name: &str) -> Option<&NetworkConfig> {
        self.networks.get(network_name)
    }

    pub fn get_script_name(&self) -> String {
        format!("{}.s.sol", self.project.script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let config_content = r#"
[project]
name = "test-contract"
script = "Deploy"
network = "sepolia"
setup_command = "bun install"

[env]
load_files = [".env"]
additional_vars = { TEST_VAR = "test_value" }

[networks.sepolia]
chain_id = 11155111
rpc_url = "https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
verify = true
"#;

        let config: DeploymentConfig = toml::from_str(config_content).unwrap();
        assert_eq!(config.project.name, "test-contract");
        assert_eq!(config.project.script, "Deploy");
        assert_eq!(config.project.network, "sepolia");
        assert_eq!(config.get_script_name(), "Deploy.s.sol");
        assert_eq!(config.get_network("sepolia").unwrap().chain_id, 11155111);
    }
}
