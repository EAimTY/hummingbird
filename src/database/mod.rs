use self::repo::FileInfo;
use crate::{Config, Data};
use anyhow::Result;
use hyper::{Body, Response};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
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
            page_files_info_map,
            post_files_info_map,
        } = repo.get_update().await?;

        let pages = Pages::from_file_info(page_files_info_map, repo.tempdir.path()).await?;
        let posts = Posts::from_file_info(post_files_info_map, repo.tempdir.path()).await?;

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
            page_files_info_map,
            post_files_info_map,
        } = self.repo.get_update().await?;

        self.theme = theme;
        self.pages = Pages::from_file_info(page_files_info_map, self.repo.tempdir.path()).await?;
        self.posts = Posts::from_file_info(post_files_info_map, self.repo.tempdir.path()).await?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub async fn init() -> Result<Self> {
        println!("Initializing database...");
        let data = DatabaseData::init().await?;
        println!("Database Initialization finished.");

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
        })
    }

    pub async fn update(&mut self) -> Result<()> {
        let mut db = self.data.write().await;
        db.update().await?;

        Ok(())
    }

    pub async fn get_page(&self, path: &str) -> Option<Response<Body>> {
        let db = self.data.read().await;
        db.pages.get(path).map(|page| db.theme.render(page))
    }

    pub async fn get_post(&self, path: &str) -> Option<Response<Body>> {
        let db = self.data.read().await;
        db.posts.get(path).map(|post| db.theme.render(post))
    }

    pub async fn get_index(&self) -> Option<Response<Body>> {
        let db = self.data.read().await;
        db.posts
            .get_index(
                Config::read().settings.index_posts_count,
                Config::read().settings.index_posts_from_old_to_new,
            )
            .map(|index| db.theme.render(index))
    }

    pub async fn get_author(&self, path: &str) -> Option<Response<Body>> {
        let db = self.data.read().await;
        db.posts
            .get_author(path)
            .map(|author| db.theme.render(author))
    }

    pub async fn not_found(&self) -> Response<Body> {
        let db = self.data.read().await;
        db.theme.render(Data::NotFound)
    }
}

#[derive(Debug)]
pub struct Update {
    pub theme: Theme,
    pub page_files_info_map: HashMap<PathBuf, FileInfo>,
    pub post_files_info_map: HashMap<PathBuf, FileInfo>,
}
