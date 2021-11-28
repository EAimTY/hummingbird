use std::env;

pub use crate::{
    config::{Config, ConfigBuilder},
    database::Database,
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

    match Database::init().await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match server::start().await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
