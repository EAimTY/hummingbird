use crate::{Config, Data};
use anyhow::Result;
use hyper::{Body, Response};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use self::{pages::Pages, posts::Posts, repo::Repo, theme::Theme, update::Update};

pub mod data;
mod pages;
mod posts;
mod repo;
mod theme;
mod update;

struct DatabaseData {
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
            pages,
            posts,
        } = repo.get_update().await?;

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
        let db = self.data.read().await;
        db.pages
            .get(path)
            .map(|page| db.theme.render(Data::Page(page)))
    }

    pub async fn get_post(&self, path: &str) -> Option<Response<Body>> {
        let db = self.data.read().await;
        db.posts
            .get(path)
            .map(|post| db.theme.render(Data::Post(post)))
    }

    pub async fn get_index(&self) -> Option<Response<Body>> {
        let db = self.data.read().await;
        db.posts
            .get_index(
                Config::read().settings.index_posts_count,
                Config::read().settings.index_posts_from_old_to_new,
            )
            .map(|list| db.theme.render(Data::List(list)))
    }
}
