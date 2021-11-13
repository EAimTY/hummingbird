use crate::{config::Config, database::Database, handler};
use anyhow::Error;
use axum::{routing::get, AddExtensionLayer, Router, Server};
use std::{process, sync::Arc};
use tokio::sync::RwLock;

pub fn start(database: Arc<RwLock<Database>>) {
    tokio::spawn(async move {
        match database.write().await.update().await {
            Ok(()) => {}
            Err(err) => exit(err),
        }

        let app = Router::new()
            .route("/:post", get(handler::post::get))
            .route("/update", get(handler::update::get))
            .layer(AddExtensionLayer::new(database));

        match Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
        {
            Ok(()) => {}
            Err(err) => exit(Error::new(err)),
        }
    });
}

fn exit(err: Error) {
    eprintln!("{}", err);
    process::exit(1);
}
