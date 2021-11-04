use crate::database::Database;
use axum::extract::{Extension, Path};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle_get(
    Path(path): Path<String>,
    Extension(database): Extension<Arc<RwLock<Database>>>,
) -> String {
    if path.ends_with(".html") {
        database.read().await.get_post(&path[..path.len() - 5])
    } else {
        "not found".into()
    }
}
