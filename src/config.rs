use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CanineConfig {
    pub host: Option<String>,
    pub token: Option<String>,
    pub account: Option<String>,
}

impl CanineConfig {
    pub const DEFAULT_HOST: &'static str = "https://canine.sh";

    pub fn credential_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".k9/kubeconfig.yaml")
    }

    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".k9/canine.yaml")
    }

    fn gate_directory(path: &Path) {
        let dir = path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(dir).expect("Failed to create parent directory");
    }

    pub fn load() -> Self {
        Self::gate_directory(&Self::config_path());

        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(Self::config_path())
            .unwrap_or_else(|_| panic!("failed to open {}",
                Self::config_path().to_str().unwrap()));

        let contents = std::fs::read_to_string(Self::config_path()).unwrap_or_else(|_| panic!("failed to read {}",
            Self::config_path().to_str().unwrap()));

        let config: CanineConfig = serde_yaml::from_str(&contents).unwrap_or_else(|_| panic!("failed to parse {}",
            Self::config_path().to_str().unwrap()));

        config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(fs::write(Self::config_path(), yaml)?)
    }

    pub fn clear() {
        Self::gate_directory(&Self::config_path());
        fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Self::config_path())
            .unwrap();
    }

    pub fn change_account(&self, account: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = CanineConfig {
            host: self.host.clone(),
            token: self.token.clone(),
            account: Some(account.to_string()),
        };
        config.save()
    }

    pub fn save_kubeconfig(&self, yaml: String) -> Result<(), Box<dyn std::error::Error>> {
        Self::gate_directory(&Self::config_path());
        fs::write(Self::credential_path(), yaml)?;
        println!(
            "{} Kubeconfig saved to {}",
            "✓".green(),
            Self::credential_path().to_str().unwrap().cyan()
        );
        Ok(())
    }
}
