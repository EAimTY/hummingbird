use crate::{config::Config, database::Database};
use once_cell::sync::OnceCell;
use std::env;

mod config;
mod database;
mod git;
mod router;

static CONFIG: OnceCell<Config> = OnceCell::new();

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::parse(args) {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    CONFIG.set(config).unwrap();

    let (database, repo_daemon) = Database::init().await;

    router::start(database).await;

    repo_daemon.listen().await;
}
