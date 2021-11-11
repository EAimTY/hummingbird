use crate::{config::Config, database::Database};
use anyhow::Result;
use axum::{handler::get, AddExtensionLayer, Router};
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle};

mod posts;
mod update;

pub fn start(database: Arc<RwLock<Database>>) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        database.write().await.update().await?;

        let app = Router::new()
            .route("/:path", get(posts::handle_get))
            .route("/update", get(update::handle))
            .layer(AddExtensionLayer::new(database));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await?;
        Ok(())
    })
}
