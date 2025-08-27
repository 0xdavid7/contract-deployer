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

#[derive(Debug)]
struct DeploymentContext {
    /// The working directory where deployment will happen
    working_directory: String,
    /// Optional path to cleanup after deployment (for temporary directories)
    cleanup_path: Option<String>,
}

impl ContractDeployer {
    pub fn new(config_path: &str) -> Result<Self> {
        let config = DeploymentConfig::from_file(config_path)?;
        let env = Environment::new();

        Ok(ContractDeployer { config, env })
    }

    pub async fn deploy(&mut self, extra_args: &[String]) -> Result<()> {
        let deployment_context = self.prepare_deployment_context().await?;

        // Execute the deployment workflow
        self.execute_deployment_workflow(&deployment_context, extra_args)
            .await?;

        // Cleanup if needed
        if let Some(cleanup_path) = &deployment_context.cleanup_path {
            self.cleanup(cleanup_path)?;
        }

        Ok(())
    }

    /// Prepare the deployment context (clone repo if needed, determine working directory)
    async fn prepare_deployment_context(&self) -> Result<DeploymentContext> {
        match &self.config.project.repo {
            Some(repo_url) => {
                let work_dir = self.prepare_repo_deployment(repo_url).await?;
                Ok(DeploymentContext {
                    working_directory: work_dir.clone(),
                    cleanup_path: Some(work_dir),
                })
            }
            None => {
                let current_dir = std::env::current_dir()
                    .context("Failed to get current directory")?
                    .to_string_lossy()
                    .to_string();

                Ok(DeploymentContext {
                    working_directory: current_dir,
                    cleanup_path: None,
                })
            }
        }
    }

    /// Prepare deployment from repository (clone and setup directory)
    async fn prepare_repo_deployment(&self, repo_url: &str) -> Result<String> {
        let base_path = self.get_deployment_base_path();
        let temp_dir = format!("{}/{}", base_path, self.config.project.name);

        println!(
            "{}",
            format!("Preparing deployment directory: {}", temp_dir).blue()
        );

        // Clone repository
        self.clone_repo(repo_url, &temp_dir).await?;

        Ok(temp_dir)
    }

    /// Get the base path for deployments
    fn get_deployment_base_path(&self) -> String {
        self.config
            .project
            .path
            .as_ref()
            .map(|p| self.sanitize_path(p))
            .unwrap_or_else(|| "/tmp".to_string())
    }

    /// Sanitize path by removing quotes and trimming
    fn sanitize_path(&self, path: &str) -> String {
        path.trim()
            .trim_matches('"')
            .trim_matches('\'')
            .trim_matches('`')
            .to_string()
    }

    /// Execute the main deployment workflow
    async fn execute_deployment_workflow(
        &mut self,
        context: &DeploymentContext,
        extra_args: &[String],
    ) -> Result<()> {
        println!(
            "{}",
            format!("Starting deployment in: {}", context.working_directory).green()
        );

        // Load environment configuration
        self.load_and_validate_environment()?;

        // Setup project (install dependencies)
        self.setup_project(&context.working_directory).await?;

        // Deploy contract
        self.deploy_contract(&context.working_directory, extra_args)
            .await?;

        Ok(())
    }

    /// Load environment configuration and validate required variables
    fn load_and_validate_environment(&mut self) -> Result<()> {
        println!("{}", "Loading environment configuration...".blue());

        // Load environment configuration
        self.env.load_from_config(&self.config.env)?;

        // Validate required environment variables
        self.validate_environment()?;

        println!(
            "{}",
            "Environment validation completed successfully!".green()
        );
        Ok(())
    }

    /// Clean up temporary files and directories
    fn cleanup(&self, cleanup_path: &str) -> Result<()> {
        println!("{}", format!("Cleaning up: {}", cleanup_path).yellow());

        fs::remove_dir_all(cleanup_path).context("Failed to cleanup temporary directory")?;

        println!("{}", "Cleanup completed successfully!".green());
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

        let mut child = Command::new(setup_parts[0])
            .args(&setup_parts[1..])
            .current_dir(project_dir)
            .stdout(std::process::Stdio::inherit()) // Show stdout in real-time
            .stderr(std::process::Stdio::inherit()) // Show stderr in real-time
            .spawn()
            .context("Failed to run setup command")?;

        let status = child
            .wait()
            .context("Failed to wait for setup command completion")?;

        if status.success() {
            println!("\n{}", "Setup command executed successfully!".green());
        } else {
            println!("\n{}", "Setup command execution failed!".red());
            if let Some(code) = status.code() {
                println!("Exit code: {}", code);
            }
            anyhow::bail!("Setup command execution failed with status: {}", status);
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

        forge_cmd.arg("--resume");

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

        for (key, value) in self.env.get_vars() {
            if key.contains("API_KEY") {
                println!("{}: {}", key.blue(), "********".yellow());
            } else if key.contains("RPC_URL") {
                println!("{}: {}", key.blue(), value);
            }
        }

        println!("{}", "═══════════════════════════════════════════════════════════════════════════════════════".green());
        println!();
    }

    fn display_command_info(&self, network_config: &NetworkConfig, script_name: &str) {
        let cmd_display = format!(
            "forge script script/{} --chain-id {} --rpc-url {} --broadcast --sender {} {}",
            script_name,
            network_config.chain_id,
            self.config.project.network,
            self.env
                .get("BROADCAST_ACCOUNT")
                .unwrap_or(&("".to_string())),
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
        println!("{}", "Executing forge script...".blue());

        // Use spawn + wait instead of output() to see real-time logs
        let mut child = forge_cmd
            .stdout(std::process::Stdio::inherit()) // Show stdout in real-time
            .stderr(std::process::Stdio::inherit()) // Show stderr in real-time
            .spawn()
            .context("Failed to start forge script")?;

        let status = child
            .wait()
            .context("Failed to wait for forge script completion")?;

        if status.success() {
            println!("\n{}", "Script executed successfully!".green());
        } else {
            println!("\n{}", "Script execution failed!".red());
            if let Some(code) = status.code() {
                println!("Exit code: {}", code);
            }
            anyhow::bail!("Script execution failed with status: {}", status);
        }

        Ok(())
    }

    fn validate_environment(&self) -> Result<()> {
        let required_vars = vec!["KEYSTORE_ACCOUNT", "KEYSTORE_PASSWORD", "BROADCAST_ACCOUNT"];

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

[env.vars]
KEYSTORE_ACCOUNT = "deployer"
KEYSTORE_PASSWORD = "****"
BROADCAST_ACCOUNT = "0xaa31349a2eF4A37Dc4Dd742E3b0E32182F524A6A"

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
