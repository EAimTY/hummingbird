use self::git::GitFileInfo;
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::{
    data_type::DataType,
    git::Repo,
    page::{Page, Pages},
    post::{Post, Posts},
    theme::Theme,
};

mod git;
mod page;
mod post;
mod theme;

pub mod data_type;

static DATABASE: OnceCell<Arc<RwLock<Database>>> = OnceCell::new();

pub struct Database {
    pub repo: Repo,
    pub theme: Theme,
    pub posts: Posts,
    pub pages: Pages,
}

impl Database {
    pub async fn init() -> Result<Database> {
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
pub struct DatabaseManager;

impl DatabaseManager {
    pub async fn init() -> Result<()> {
        println!("Initializing database...");

        let data = Database::init().await?;
        DATABASE
            .set(Arc::new(RwLock::new(data)))
            .map_err(|_| anyhow!("Failed to initialize database"))?;

        println!("Database Initialization finished.");

        Ok(())
    }

    pub async fn read() -> RwLockReadGuard<'static, Database> {
        let db_lock = DATABASE.get().unwrap();
        db_lock.read().await
    }

    pub async fn write() -> RwLockWriteGuard<'static, Database> {
        let db_lock = DATABASE.get().unwrap();
        db_lock.write().await
    }
}

#[derive(Debug)]
pub struct Update {
    pub theme: Theme,
    pub page_files_info_map: HashMap<PathBuf, GitFileInfo>,
    pub post_files_info_map: HashMap<PathBuf, GitFileInfo>,
}
