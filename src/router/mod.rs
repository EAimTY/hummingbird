use crate::{config::Config, database::Database};
use axum::{handler::get, AddExtensionLayer, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

mod posts;
mod update;

pub async fn start(database: Arc<RwLock<Database>>) {
    tokio::spawn(async move {
        database.write().await.update().await;

        let app = Router::new()
            .route("/:path", get(posts::handle_get))
            .route("/update", get(update::handle))
            .layer(AddExtensionLayer::new(database));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}
