use crate::{
    data::{Data, PostData},
    database::Database,
};
use axum::extract::{Extension, Path};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle_get(
    Path(path): Path<String>,
    Extension(database): Extension<Arc<RwLock<Database>>>,
) -> String {
    if path.ends_with(".html") {
        let post = database
            .read()
            .await
            .get_post(path[..path.len() - 5].to_string());

        database
            .read()
            .await
            .theme
            .render(Data::Post(PostData { data: post }))
    } else {
        "not found".into()
    }
}
