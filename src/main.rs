use crate::config::Config;
use std::env;
use tokio::sync::mpsc;

mod config;
mod db;
mod op;
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

    let (op_sender, mut op_reciever) = mpsc::channel(8);

    router::Router::run(&config.settings, op_sender)
        .await
        .unwrap();

    match db::Db::init(&config) {
        Ok(db) => db.listen(op_reciever).await,
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
}
