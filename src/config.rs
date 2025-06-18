use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub tools: ToolsConfig,
    pub build: BuildConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ToolsConfig {
    pub custom_tools: Vec<String>,
    pub check_paths: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BuildConfig {
    pub release_by_default: bool,
    pub show_warnings: bool,
    pub target_dir: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OutputConfig {
    pub json_by_default: bool,
    pub color: bool,
}

impl Config {
    #[allow(dead_code)]
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", config_path))
    }

    #[allow(dead_code)]
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Failed to get config directory")?;
        Ok(config_dir.join("oxygen").join("config.toml"))
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }
}
