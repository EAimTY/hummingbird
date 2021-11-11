use crate::{config::Config, database::Database, handler};
use anyhow::Result;
use axum::{handler::get, AddExtensionLayer, Router};
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle};

pub fn start(database: Arc<RwLock<Database>>) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        database.write().await.update().await?;

        let app = Router::new()
            .route("/:path", get(handler::post::get))
            .route("/update", get(handler::update::get))
            .layer(AddExtensionLayer::new(database));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await?;

        Ok(())
    })
}
