use crate::{config::Config, database::Database};
use std::env;

mod config;
mod database;
mod router;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::parse(args) {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    let (database, repo_daemon) = Database::init().await;

    router::start(database).await;

    repo_daemon.listen().await;
}
