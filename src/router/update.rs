use crate::database::Database;
use axum::extract::Extension;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle(Extension(database): Extension<Arc<RwLock<Database>>>) -> String {
    match database.write().await.update().await {
        Ok(()) => String::from("OK"),
        Err(err) => err.to_string(),
    }
}
