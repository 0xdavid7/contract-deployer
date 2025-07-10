use crate::config::EnvConfig;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Environment {
    vars: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn load_from_config(&mut self, config: &EnvConfig, project_dir: &str) -> Result<()> {
        // Load environment files
        for file_path in &config.load_files {
            let full_path = Path::new(project_dir).join(file_path);
            if full_path.exists() {
                self.load_env_file(&full_path)?;
            }
        }

        // Load additional variables from config
        if let Some(additional_vars) = &config.additional_vars {
            for (key, value) in additional_vars {
                self.vars.insert(key.clone(), value.clone());
            }
        }

        // Load system environment variables (override file variables)
        for (key, value) in env::vars() {
            self.vars.insert(key, value);
        }

        Ok(())
    }

    fn load_env_file(&mut self, file_path: &Path) -> Result<()> {
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read env file: {:?}", file_path))?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"').trim_matches('\'');
                self.vars.insert(key.to_string(), value.to_string());
            }
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    pub fn get_vars(&self) -> &HashMap<String, String> {
        &self.vars
    }

    pub fn expand_variables(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Replace ${VAR} patterns
        while let Some(start) = result.find("${") {
            if let Some(end) = result[start..].find('}') {
                let var_name = &result[start + 2..start + end];
                let empty_string = "".to_string();
                let replacement = self.get(var_name).unwrap_or(&empty_string);
                result.replace_range(start..start + end + 1, replacement);
            } else {
                break;
            }
        }

        result
    }

    pub fn validate_required(&self, required_vars: &[&str]) -> Result<()> {
        let mut missing = Vec::new();

        for &var in required_vars {
            if !self.vars.contains_key(var) {
                missing.push(var);
            }
        }

        if !missing.is_empty() {
            anyhow::bail!(
                "Missing required environment variables: {}",
                missing.join(", ")
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_env_variable_expansion() {
        let mut env = Environment::new();
        env.vars
            .insert("API_KEY".to_string(), "test123".to_string());
        env.vars
            .insert("NETWORK".to_string(), "sepolia".to_string());

        let expanded = env.expand_variables("https://eth-${NETWORK}.g.alchemy.com/v2/${API_KEY}");
        assert_eq!(expanded, "https://eth-sepolia.g.alchemy.com/v2/test123");
    }

    #[test]
    fn test_env_file_loading() {
        let temp_dir = tempdir().unwrap();
        let env_file = temp_dir.path().join(".env");

        fs::write(
            &env_file,
            "TEST_KEY=test_value\n# Comment\nANOTHER_KEY=another_value",
        )
        .unwrap();

        let mut env = Environment::new();
        env.load_env_file(&env_file).unwrap();

        assert_eq!(env.get("TEST_KEY"), Some(&"test_value".to_string()));
        assert_eq!(env.get("ANOTHER_KEY"), Some(&"another_value".to_string()));
    }

    #[test]
    fn test_validation() {
        let mut env = Environment::new();
        env.vars
            .insert("EXISTING_VAR".to_string(), "value".to_string());

        // Should pass with existing variable
        assert!(env.validate_required(&["EXISTING_VAR"]).is_ok());

        // Should fail with missing variable
        assert!(env.validate_required(&["MISSING_VAR"]).is_err());

        // Should fail with mix of existing and missing
        assert!(
            env.validate_required(&["EXISTING_VAR", "MISSING_VAR"])
                .is_err()
        );
    }
}
