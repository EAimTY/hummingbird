pub use self::{data::Data, pages::Pages, posts::Posts, repo::Repo, theme::Theme, update::Update};
use std::sync::Arc;
use tokio::sync::RwLock;

mod data;
mod init;
mod pages;
mod posts;
mod repo;
mod theme;
mod update;

pub struct DatabaseData {
    repo: Repo<'static>,
    theme: Theme,
    posts: Posts,
    pages: Pages,
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

    pub async fn get_page(&self, path: &str) -> String {
        let database = self.data.read().await;
        let page = database.pages.get_page(path);
        database.theme.render(Data::Page(page))
    }
}
