use self::git::ParsedGitRepo;
use crate::RouteTable;
use anyhow::{anyhow, Error, Result};
use once_cell::sync::OnceCell;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::{
    author::Authors,
    git::Repo,
    page::{Page, Pages},
    post::{Post, Posts},
    theme::Theme,
};

mod author;
mod git;
mod page;
mod post;
mod theme;

static DATABASE: OnceCell<RwLock<Database>> = OnceCell::new();

pub struct Database {
    pub repo: Repo,
    pub theme: Theme,
    pub posts: Posts,
    pub pages: Pages,
    pub authors: Authors,
}

impl Database {
    pub async fn init() -> Result<Database> {
        let mut repo = Repo::init()?;

        let ParsedGitRepo {
            theme,
            pages_git_file_info,
            posts_git_file_info,
        } = repo.parse().await?;

        let pages = Pages::from_git_file_info(pages_git_file_info, repo.tempdir.path()).await?;
        let posts = Posts::from_git_file_info(posts_git_file_info, repo.tempdir.path()).await?;
        let authors = Authors::generate(&pages, &posts);

        Ok(Self {
            repo,
            theme,
            posts,
            pages,
            authors,
        })
    }

    pub async fn update(&mut self) -> Result<()> {
        let ParsedGitRepo {
            theme,
            pages_git_file_info,
            posts_git_file_info,
        } = self.repo.parse().await?;

        RouteTable::clear_path_map().await?;

        self.theme = theme;
        self.pages =
            Pages::from_git_file_info(pages_git_file_info, self.repo.tempdir.path()).await?;
        self.posts =
            Posts::from_git_file_info(posts_git_file_info, self.repo.tempdir.path()).await?;
        self.authors = Authors::generate(&self.pages, &self.posts);

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
            .set(RwLock::new(data))
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

pub enum DatabaseUpdateResult {
    Success,
    PermissionDenied,
    Error(Error),
}
