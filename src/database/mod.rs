pub use self::{data::Data, git::Repo, post::Posts, theme::Theme, update::Update};
use std::sync::Arc;
use tokio::sync::RwLock;

mod data;
mod git;
mod init;
mod post;
mod theme;
mod update;

pub struct DatabaseData {
    theme: Theme,
    posts: Posts,
    repo: Repo<'static>,
}

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub async fn get_post(&self, path: &str) -> String {
        let database = self.data.read().await;
        let post = database.posts.get_post(path);
        database.theme.render(Data::Post(post))
    }
}
