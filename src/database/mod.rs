use self::theme::Theme;
pub use self::{
    archive::Archive,
    post::{Post, Posts},
};
use crate::{
    config::Config,
    git::{Repo, RepoDaemon},
};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};

mod archive;
mod post;
mod theme;

pub struct Database {
    pub theme: Theme,
    pub posts: Posts,
    repo_update_sender: mpsc::Sender<oneshot::Sender<DatabaseUpdate>>,
}

impl Database {
    pub async fn init() -> (Arc<RwLock<Self>>, RepoDaemon<'static>) {
        let (repo_update_sender, repo_update_listener) = mpsc::channel(1);

        (
            Arc::new(RwLock::new(Self {
                theme: Theme::new(),
                posts: Posts::new(),
                repo_update_sender,
            })),
            Repo::init(repo_update_listener),
        )
    }

    pub async fn update(&mut self) {
        let (update_sender, update_receiver) = oneshot::channel();
        self.repo_update_sender.send(update_sender).await.unwrap();
        let DatabaseUpdate { posts } = update_receiver.await.unwrap();
        self.posts = posts;
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
