use crate::{config::ConfigBuilder, database::Database};
use std::env;

mod config;
mod database;
mod router;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config_builder = ConfigBuilder::new();

    match config_builder.parse(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}\n\n{}", err, config_builder.get_usage());
            return;
        }
    }

    let database = match Database::init().await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    server::start(database).await;
}
