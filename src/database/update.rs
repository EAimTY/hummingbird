use crate::{
    database::{Pages, Posts, Theme},
    Database,
};
use anyhow::Result;

impl Database {
    pub async fn update(&mut self) -> Result<()> {
        let mut db = self.data.write().await;

        let Update {
            theme,
            posts,
            pages,
        } = db.repo.get_update().await?;

        db.theme = theme;
        db.posts = posts;
        db.pages = pages;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Update {
    pub theme: Theme,
    pub posts: Posts,
    pub pages: Pages,
}
