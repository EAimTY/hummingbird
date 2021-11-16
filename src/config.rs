use anyhow::{anyhow, bail, Result};
use getopts::Options;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, path::Path};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub git: Git,
    pub settings: Settings,
    pub url_patterns: UrlPatterns,
}

#[derive(Debug, Deserialize)]
pub struct Git {
    pub repository: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub update_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UrlPatterns {
    pub post_url: String,
    pub page_url: String,
}

impl Config {
    fn from_file(config_file: String) -> Result<Config> {
        let config = fs::read_to_string(Path::new(&config_file))?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }

    pub fn read() -> &'static Self {
        CONFIG.get().expect("config is not initialized")
    }
}

pub struct ConfigBuilder<'cfg> {
    opts: Options,
    program: Option<&'cfg str>,
}

impl<'cfg> ConfigBuilder<'cfg> {
    pub fn new() -> Self {
        let mut opts = Options::new();
        opts.optopt("c", "config-file", "config file path", "CONFIG");
        opts.optflag("h", "help", "print the help menu");

        ConfigBuilder {
            opts,
            program: None,
        }
    }

    pub fn get_usage(&self) -> String {
        self.opts
            .usage(&format!("Usage: {} [options]", self.program.unwrap()))
    }

    pub fn parse(&mut self, args: &'cfg Vec<String>) -> Result<()> {
        self.program = Some(&args[0]);

        let matches = self.opts.parse(&args[1..])?;

        if !matches.free.is_empty() {
            bail!("unexpected arguments: {}", matches.free.join(", "));
        }

        if matches.opt_present("h") {
            bail!("");
        }

        let config_file = matches
            .opt_str("c")
            .ok_or_else(|| anyhow!("No config file specificed"))?;

        let config = Config::from_file(config_file)?;
        CONFIG.set(config).unwrap();

        Ok(())
    }
}
