use self::git::ParsedGitRepo;
use crate::{Config, RouteTable};
use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
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

        RouteTable::clear().await;

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

pub enum TimeRange {
    Year {
        year: i32,
        from: DateTime<Tz>,
        to: DateTime<Tz>,
    },
    Month {
        year: i32,
        month: u32,
        from: DateTime<Tz>,
        to: DateTime<Tz>,
    },
}

impl TimeRange {
    pub fn parse(year: &str, month: Option<&str>) -> Option<Self> {
        let tz = &Config::read().settings.timezone;

        let year = year.parse().ok()?;

        if let Some(month) = month {
            let month = month.parse().ok()?;

            let from = tz.ymd_opt(year, month, 1).single()?.and_hms(0, 0, 0);

            let (to_year, to_month) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };

            let to = tz.ymd_opt(to_year, to_month, 1).single()?.and_hms(0, 0, 0);

            return Some(Self::Month {
                year,
                month,
                from,
                to,
            });
        }

        let from = tz.ymd_opt(year, 1, 1).single()?.and_hms(0, 0, 0);
        let to = tz.ymd_opt(year + 1, 1, 1).single()?.and_hms(0, 0, 0);

        Some(Self::Year { year, from, to })
    }

    pub fn from(&self) -> &DateTime<Tz> {
        match self {
            Self::Year { from, .. } => from,
            Self::Month { from, .. } => from,
        }
    }

    pub fn to(&self) -> &DateTime<Tz> {
        match self {
            Self::Year { to, .. } => to,
            Self::Month { to, .. } => to,
        }
    }
}
