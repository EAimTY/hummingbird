use super::{Database, Pages, Posts, Theme};
use anyhow::Result;

impl Database {
    pub async fn update(&mut self) -> Result<()> {
        let mut database = self.data.write().await;

        let Update {
            theme,
            posts,
            pages,
        } = database.repo.get_update().await?;

        database.theme = theme;
        database.posts = posts;
        database.pages = pages;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Update {
    pub theme: Theme,
    pub posts: Posts,
    pub pages: Pages,
}
