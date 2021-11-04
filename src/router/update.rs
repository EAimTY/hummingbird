use crate::database::Database;
use axum::extract::Extension;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle(Extension(database): Extension<Arc<RwLock<Database>>>) -> &'static str {
    database.write().await.update().await;

    "done"
}