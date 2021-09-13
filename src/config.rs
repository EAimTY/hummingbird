use anyhow::{bail, Context, Result};
use getopts::Options;
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
    pub fn parse(args: Vec<String>) -> Result<Self> {
        let mut opts = Options::new();

        opts.optopt("c", "config-file", "config file path", "CONFIG");
        opts.optflag("h", "help", "print the help menu");

        let usage = opts.usage(&format!("Usage: {} [options]", args[0]));

        let matches = opts
            .parse(&args[1..])
            .context(format!("Failed to parse arguments\n{}", usage.clone()))?;

        if !matches.free.is_empty() {
            bail!("Unexpected fragment\n{}", usage.clone());
        }

        if matches.opt_present("h") {
            bail!("{}", usage.clone());
        }

        let config_file = matches
            .opt_str("c")
            .context(format!("No config file specificed\n{}", usage.clone()))?;

        Self::from_file(config_file)
    }

    fn from_file(config_file: String) -> Result<Config> {
        let config = fs::read_to_string(Path::new(&config_file))?;

        let config = toml::from_str(&config)?;

        Ok(config)
    }
}
