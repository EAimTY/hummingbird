use crate::config::Config;
use std::env;
use tokio::sync::mpsc;

mod config;
mod db;
mod repo;
mod router;

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

    let (tx, mut rx) = mpsc::channel(32);

    router::Router::run(&config.settings, tx).await.unwrap();

    match db::Db::init(&config) {
        Ok(db) => db.listen(rx).await,
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
}
