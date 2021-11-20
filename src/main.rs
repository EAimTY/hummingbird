use std::env;

pub use crate::{
    config::{Config, ConfigBuilder},
    database::{
        data::{self, Data},
        Database,
    },
    router::Router,
};

mod config;
mod database;
mod router;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cfg_builder = ConfigBuilder::new();

    match cfg_builder.parse(&args) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}\n\n{}", err, cfg_builder.get_usage());
            return;
        }
    }

    let db = match Database::init().await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match server::start(db).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
