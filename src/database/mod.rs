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

pub struct Database {
    pub theme: Theme,
    pub posts: Posts,
    repo_update_sender: mpsc::Sender<oneshot::Sender<DatabaseUpdate>>,
}

impl Database {
    pub async fn init() -> Result<(Arc<RwLock<Self>>, RepoDaemon<'static>)> {
        let (repo_update_sender, repo_update_listener) = mpsc::channel(1);

        Ok((
            Arc::new(RwLock::new(Self {
                theme: Theme::new(),
                posts: Posts::new(),
                repo_update_sender,
            })),
            Repo::init(repo_update_listener)?,
        ))
    }

    pub async fn update(&mut self) -> Result<()> {
        let (update_sender, update_receiver) = oneshot::channel();
        self.repo_update_sender.send(update_sender).await?;
        let DatabaseUpdate { posts } = update_receiver.await?;
        self.posts = posts;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DatabaseUpdate {
    pub posts: Posts,
}

pub enum Query<'query> {
    Post(&'query Post),
    Archive(Archive<'query>),
    // ...
}
