use super::{Database, DatabaseData, Repo, Update};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

impl Database {
    pub async fn init() -> Result<Self> {
        let mut repo = Repo::init()?;
        let Update { posts, theme } = repo.get_update().await;

        Ok(Self {
            data: Arc::new(RwLock::new(DatabaseData { repo, theme, posts })),
        })
    }
}
