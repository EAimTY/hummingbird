use crate::{Config, Data};
use anyhow::Result;
use hyper::{Body, Response};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use self::{pages::Pages, posts::Posts, repo::Repo, theme::Theme};

mod pages;
mod posts;
mod repo;
mod theme;

pub mod data;

struct DatabaseData {
    repo: Repo,
    theme: Theme,
    posts: Posts,
    pages: Pages,
}

impl DatabaseData {
    pub async fn init() -> Result<DatabaseData> {
        let mut repo = Repo::init()?;

        let Update {
            theme,
            pages,
            posts,
        } = repo.get_update().await?;

        Ok(Self {
            repo,
            theme,
            posts,
            pages,
        })
    }

    pub async fn update(&mut self) -> Result<()> {
        let Update {
            theme,
            posts,
            pages,
        } = self.repo.get_update().await?;

        self.theme = theme;
        self.posts = posts;
        self.pages = pages;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub async fn init() -> Result<Self> {
        Ok(Self {
            data: Arc::new(RwLock::new(DatabaseData::init().await?)),
        })
    }

    pub async fn update(&mut self) -> Result<()> {
        let mut db = self.data.write().await;
        db.update().await?;

        Ok(())
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

#[derive(Debug)]
pub struct Update {
    pub theme: Theme,
    pub posts: Posts,
    pub pages: Pages,
}
