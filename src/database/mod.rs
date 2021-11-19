pub use self::{data::Data, pages::Pages, posts::Posts, repo::Repo, theme::Theme, update::Update};
use anyhow::Result;
use hyper::{Body, Response};
use std::sync::Arc;
use tokio::sync::RwLock;

mod data;
mod pages;
mod posts;
mod repo;
mod theme;
mod update;

pub struct DatabaseData {
    repo: Repo,
    theme: Theme,
    posts: Posts,
    pages: Pages,
}

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub async fn init() -> Result<Self> {
        let mut repo = Repo::init()?;
        let Update {
            theme,
            posts,
            pages,
        } = repo.get_update().await;

        Ok(Self {
            data: Arc::new(RwLock::new(DatabaseData {
                repo,
                theme,
                posts,
                pages,
            })),
        })
    }

    pub async fn get_page(&self, path: &str) -> Option<Response<Body>> {
        let database = self.data.read().await;
        database
            .pages
            .get(path)
            .map(|page| database.theme.render(Data::Page(page)))
    }

    pub async fn get_post(&self, path: &str) -> Option<Response<Body>> {
        let database = self.data.read().await;
        database
            .posts
            .get(path)
            .map(|post| database.theme.render(Data::Post(post)))
    }
}
