use self::theme::Theme;
pub use self::{
    archive::Archive,
    git::{Repo, RepoDaemon},
    post::{Post, Posts},
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};

mod archive;
mod git;
mod post;
mod theme;

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub async fn init() -> Result<(Database, RepoDaemon<'static>)> {
        let (repo_update_sender, repo_update_listener) = mpsc::channel(1);

        Ok((
            Self {
                data: Arc::new(RwLock::new(DatabaseData {
                    theme: Theme::new(),
                    posts: Posts::new(),
                    repo_update_sender,
                })),
            },
            Repo::init(repo_update_listener)?,
        ))
    }

    pub async fn update(&mut self) -> Result<()> {
        let mut database = self.data.write().await;

        let (update_sender, update_receiver) = oneshot::channel();
        database.repo_update_sender.send(update_sender).await?;

        let DatabaseUpdate { posts } = update_receiver.await?;

        database.posts = posts;

        Ok(())
    }
}

pub struct DatabaseData {
    theme: Theme,
    posts: Posts,
    repo_update_sender: mpsc::Sender<oneshot::Sender<DatabaseUpdate>>,
}

#[derive(Debug)]
pub struct DatabaseUpdate {
    pub posts: Posts,
}

pub enum Data<'data> {
    Post(&'data Post),
    Archive(Archive<'data>),
    // ...
}
