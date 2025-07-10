use anyhow::{Context, Result};
use colored::*;
use git2::Repository;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use crate::config::{DeploymentConfig, NetworkConfig};
use crate::environment::Environment;

pub struct ContractDeployer {
    config: DeploymentConfig,
    env: Environment,
}

impl ContractDeployer {
    pub fn new(config_path: &str) -> Result<Self> {
        let config = DeploymentConfig::from_file(config_path)?;
        let env = Environment::new();

        Ok(ContractDeployer { config, env })
    }

    pub async fn deploy(&mut self, extra_args: &[String]) -> Result<()> {
        match &self.config.project.repo {
            Some(url) => {
                let url_owned = url.to_string();
                let path = self
                    .config
                    .project
                    .path
                    .clone()
                    .unwrap_or("tmp".to_string());

                let path = path
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .trim_matches('`');

                self.deploy_from_repo(&url_owned, path, extra_args).await
            }
            None => {
                let current_dir = std::env::current_dir()?;
                self.deploy_local(current_dir.to_str().unwrap(), extra_args)
                    .await
            }
        }
    }

    async fn deploy_from_repo(
        &mut self,
        repo_url: &str,
        path: &str,
        extra_args: &[String],
    ) -> Result<()> {
        let temp_dir = format!("{}/{}", path, self.config.project.name);

        // Clone repository
        self.clone_repo(repo_url, &temp_dir).await?;

        // Load environment configuration
        self.env.load_from_config(&self.config.env, &temp_dir)?;

        // Validate required environment variables
        self.validate_environment()?;

        // Setup project (install dependencies)
        self.setup_project(&temp_dir).await?;

        // Deploy contract
        self.deploy_contract(&temp_dir, extra_args).await?;

        // Cleanup
        fs::remove_dir_all(&temp_dir).context("Failed to cleanup temporary directory")?;

        Ok(())
    }

    async fn deploy_local(&mut self, project_dir: &str, extra_args: &[String]) -> Result<()> {
        // Load environment configuration
        self.env.load_from_config(&self.config.env, project_dir)?;

        // Validate required environment variables
        self.validate_environment()?;

        // Setup project (install dependencies)
        self.setup_project(project_dir).await?;

        // Deploy contract
        self.deploy_contract(project_dir, extra_args).await?;

        Ok(())
    }

    async fn clone_repo(&self, repo_url: &str, target_dir: &str) -> Result<()> {
        println!("{}", "Cloning repository...".blue());

        if Path::new(target_dir).exists() {
            fs::remove_dir_all(target_dir).context("Failed to remove existing directory")?;
        }

        Repository::clone(repo_url, target_dir).context("Failed to clone repository")?;

        println!("{}", "Repository cloned successfully!".green());
        Ok(())
    }

    async fn setup_project(&self, project_dir: &str) -> Result<()> {
        println!("{}", "Setting up project...".blue());

        let setup_parts: Vec<&str> = self
            .config
            .project
            .setup_command
            .split_whitespace()
            .collect();
        if setup_parts.is_empty() {
            return Ok(());
        }

        let output = Command::new(setup_parts[0])
            .args(&setup_parts[1..])
            .current_dir(project_dir)
            .output()
            .context("Failed to run setup command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Setup command failed: {}", stderr);
        }

        println!("{}", "Project setup completed successfully!".green());
        Ok(())
    }

    async fn deploy_contract(&self, project_dir: &str, extra_args: &[String]) -> Result<()> {
        // Get network configuration
        let network_config = self
            .config
            .get_network(&self.config.project.network)
            .context(format!(
                "Network '{}' not found in configuration",
                self.config.project.network
            ))?;

        // Expand variables in RPC URL
        let rpc_url = self.env.expand_variables(&network_config.rpc_url);
        let expanded_network_config = NetworkConfig {
            chain_id: network_config.chain_id,
            rpc_url,
            verify: network_config.verify,
        };

        self.display_deployment_info(&expanded_network_config);

        let script_name = self.config.get_script_name();
        println!(
            "{}",
            format!("Running Forge script: {}", script_name).green()
        );

        // Build forge command
        let mut forge_cmd = self
            .build_forge_command(&expanded_network_config, &script_name, extra_args)
            .await?;
        forge_cmd.current_dir(project_dir);

        // Set environment variables for the forge process
        for (key, value) in self.env.get_vars() {
            forge_cmd.env(key, value);
        }

        // Display command (without sensitive info)
        self.display_command_info(&expanded_network_config, &script_name);

        // Ask for confirmation
        if !self.confirm_execution()? {
            println!("Script execution cancelled");
            return Ok(());
        }

        // Execute the command
        self.execute_forge_command(forge_cmd).await?;

        Ok(())
    }

    async fn build_forge_command(
        &self,
        network_config: &NetworkConfig,
        script_name: &str,
        extra_args: &[String],
    ) -> Result<Command> {
        let mut forge_cmd = Command::new("forge");

        forge_cmd
            .arg("script")
            .arg(format!("script/{}", script_name))
            .arg("--chain-id")
            .arg(network_config.chain_id.to_string())
            .arg("--rpc-url")
            .arg(&self.config.project.network)
            .arg("--broadcast");

        // Add verification if enabled
        if network_config.verify {
            forge_cmd.arg("--verify");
        }

        // Add account and authentication
        if let Some(keystore_account) = self.env.get("KEYSTORE_ACCOUNT") {
            forge_cmd.arg("--account").arg(keystore_account);
        }

        if let Some(keystore_password) = self.env.get("KEYSTORE_PASSWORD") {
            forge_cmd.arg("--password").arg(keystore_password);
        }

        if let Some(broadcast_account) = self.env.get("BROADCAST_ACCOUNT") {
            forge_cmd.arg("--sender").arg(broadcast_account);
        }

        // Add extra arguments
        for arg in extra_args {
            forge_cmd.arg(arg);
        }

        Ok(forge_cmd)
    }

    fn display_deployment_info(&self, network_config: &NetworkConfig) {
        println!("\n{}", "════════════════════════════════════ DEPLOYMENT CONFIG ════════════════════════════════════".green());
        println!("{}: {}", "PROJECT".blue(), self.config.project.name);
        println!("{}: {}", "SCRIPT".blue(), self.config.get_script_name());
        println!("{}: {}", "NETWORK".blue(), self.config.project.network);
        println!("{}: {}", "CHAIN_ID".blue(), network_config.chain_id);
        println!("{}: {}", "RPC_URL".blue(), network_config.rpc_url);
        println!("{}: {}", "VERIFY".blue(), network_config.verify);

        if let Some(api_key) = self.env.get("API_KEY_ETHERSCAN") {
            println!("{}: {}", "API_KEY_ETHERSCAN".blue(), api_key);
        }

        println!("{}", "═══════════════════════════════════════════════════════════════════════════════════════".green());
        println!();
    }

    fn display_command_info(&self, network_config: &NetworkConfig, script_name: &str) {
        let cmd_display = format!(
            "forge script script/{} --chain-id {} --rpc-url {} --broadcast{}",
            script_name,
            network_config.chain_id,
            self.config.project.network,
            if network_config.verify {
                " --verify"
            } else {
                ""
            }
        );
        println!("Executing: {}", cmd_display);
    }

    fn confirm_execution(&self) -> Result<bool> {
        print!("Continue with script execution? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        Ok(input == "y" || input == "yes")
    }

    async fn execute_forge_command(&self, mut forge_cmd: Command) -> Result<()> {
        let output = forge_cmd
            .output()
            .context("Failed to execute forge script")?;

        if output.status.success() {
            println!("{}", "Script executed successfully!".green());
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                println!("\nOutput:\n{}", stdout);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", "Script execution failed!".red());
            if !stdout.is_empty() {
                println!("stdout: {}", stdout);
            }
            if !stderr.is_empty() {
                println!("stderr: {}", stderr);
            }
            anyhow::bail!("Script execution failed");
        }

        Ok(())
    }

    fn validate_environment(&self) -> Result<()> {
        let required_vars = vec![
            "API_KEY_ETHERSCAN",
            "ALCHEMY_API_KEY",
            "KEYSTORE_ACCOUNT",
            "KEYSTORE_PASSWORD",
            "BROADCAST_ACCOUNT",
        ];

        self.env.validate_required(&required_vars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_deployer_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let config_content = r#"
[project]
name = "test-contract"
script = "Deploy"
network = "sepolia"
setup_command = "echo 'test'"

[env]
load_files = [".env"]

[networks.sepolia]
chain_id = 11155111
rpc_url = "https://eth-sepolia.g.alchemy.com/v2/test"
verify = true
"#;

        fs::write(&config_path, config_content).unwrap();

        let deployer = ContractDeployer::new(config_path.to_str().unwrap());
        assert!(deployer.is_ok());
    }
}
