use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
pub struct Config {
    pub git: Git,
    pub settings: Settings,
}

#[derive(Deserialize)]
pub struct Git {
    pub repository: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
}

#[derive(Deserialize)]
pub struct Settings {
    pub update_token: Option<String>,
}

impl Config {
    pub fn from(config_file: String) -> Result<Config> {
        let config = fs::read_to_string(Path::new(&config_file))
            .context("Failed to read the config file")?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }
}
