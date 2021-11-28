use self::git::GitFileInfo;
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

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

pub struct DatabaseData {
    pub repo: Repo,
    pub theme: Theme,
    pub posts: Posts,
    pub pages: Pages,
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
    pub data: Arc<RwLock<DatabaseData>>,
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
}

#[derive(Debug)]
pub struct Update {
    pub theme: Theme,
    pub page_files_info_map: HashMap<PathBuf, GitFileInfo>,
    pub post_files_info_map: HashMap<PathBuf, GitFileInfo>,
}
