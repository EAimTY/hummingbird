use super::{Database, Posts, Theme};
use anyhow::Result;

impl Database {
    pub async fn update(&mut self) -> Result<()> {
        let mut database = self.data.write().await;

        let Update { posts, theme } = database.repo.get_update().await;
        database.posts = posts;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Update {
    pub posts: Posts,
    pub theme: Theme,
}
