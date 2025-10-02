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
    pub extra_args: Option<HashMap<String, String>>,
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

[extra_args]
gas-limit = "1000000"
priority-fee = "2"
legacy = ""

[env]
# Environment files to load (in order, later files override earlier ones)
load_files = [".env"]

# Additional environment variables can be set directly in config
# These will override variables from files
[env.vars]
KEYSTORE_ACCOUNT = "deployer"
KEYSTORE_PASSWORD = "****"
BROADCAST_ACCOUNT = "0xaa31349a2eF4A37Dc4Dd742E3b0E32182F524A6A"

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

        // Test args parsing
        let args = config.extra_args;
        if let None = args {
            assert!(false, "args should be Some");
        }
        let args = args.unwrap();
        let entry1 = args.get("gas-limit");
        assert_eq!(entry1, Some(&"1000000".to_string()));
        let entry2 = args.get("priority-fee");
        assert_eq!(entry2, Some(&"2".to_string()));
        let entry3 = args.get("legacy");
        assert_eq!(entry3, Some(&"".to_string()));

        // legacy has empty value, so only the flag should be present
    }
}
