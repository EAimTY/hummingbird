pub use self::post::Post;
use self::theme::Theme;
use crate::{
    config::Config,
    git::{Repo, RepoDaemon},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, oneshot, RwLock};

mod post;
mod theme;

pub struct Database {
    pub theme: Theme,
    pub posts: HashMap<String, Post>,
    repo_update_sender: mpsc::Sender<oneshot::Sender<DatabaseUpdate>>,
}

impl Database {
    pub async fn init(config: &Config) -> (Arc<RwLock<Self>>, RepoDaemon<'_>) {
        let (repo_update_sender, repo_update_listener) = mpsc::channel(1);

        (
            Arc::new(RwLock::new(Self {
                theme: Theme::new(),
                posts: HashMap::new(),
                repo_update_sender,
            })),
            Repo::init(config, repo_update_listener),
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
    pub posts: HashMap<String, Post>,
}
