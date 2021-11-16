pub use self::{
    archive::Archive,
    git::Repo,
    post::{Post, Posts},
    theme::Theme,
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

mod archive;
mod git;
mod post;
mod theme;

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub async fn init() -> Result<Self> {
        Ok(Self {
            data: Arc::new(RwLock::new(DatabaseData {
                theme: Theme::new(),
                posts: Posts::new(),
                repo: Repo::init()?,
            })),
        })
    }

    pub async fn update(&mut self) -> Result<()> {
        let mut database = self.data.write().await;

        let DatabaseUpdate { posts } = database.repo.update().await;

        database.posts = posts;

        Ok(())
    }
}

pub struct DatabaseData {
    theme: Theme,
    posts: Posts,
    repo: Repo<'static>,
}

unsafe impl Send for DatabaseData {}
unsafe impl Sync for DatabaseData {}

#[derive(Debug)]
pub struct DatabaseUpdate {
    pub posts: Posts,
}

pub enum Data<'data> {
    Post(&'data Post),
    Archive(Archive<'data>),
    // ...
}
