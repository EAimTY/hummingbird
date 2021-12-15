use anyhow::{anyhow, bail, Result};
use chrono_tz::Tz;
use getopts::Options;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, net::SocketAddr, path::Path};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub application: Application,
    pub git: Git,
    pub site: Site,
    pub url_patterns: UrlPatterns,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub listen: SocketAddr,
    pub timezone: Tz,
    pub update_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Git {
    pub repository: String,
    pub branch: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub homepage: String,
    pub list_posts_count: usize,
    pub list_from_old_to_new: bool,
}

#[derive(Debug, Deserialize)]
pub struct UrlPatterns {
    pub index: String,
    pub update: String,
    pub page: String,
    pub post: String,
    pub author: String,
    pub archive: String,
    pub search: String,
}

impl Config {
    fn from_file(config_file: String) -> Result<Config> {
        let config = fs::read_to_string(Path::new(&config_file))?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }

    fn process(&mut self) {
        self.site.url = self.site.url.trim_end_matches('/').to_owned();
    }

    pub fn read() -> &'static Self {
        CONFIG.get().unwrap()
    }
}

pub struct ConfigBuilder<'cfg> {
    opts: Options,
    program: Option<&'cfg str>,
}

impl<'cfg> ConfigBuilder<'cfg> {
    #[allow(clippy::new_without_default)]
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

    pub fn parse(&mut self, args: &'cfg [String]) -> Result<()> {
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

        let mut config = Config::from_file(config_file)?;
        config.process();

        CONFIG.set(config).unwrap();

        Ok(())
    }
}
