use self::git::ParsedGitRepo;
use crate::{Config, RouteTable};
use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use once_cell::sync::OnceCell;
use std::fmt::{self, Display, Formatter};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::{
    author::Authors,
    git::Repo,
    page::{Page, Pages},
    post::{Post, PostFilter, Posts},
    template::Template,
};

mod author;
mod git;
mod page;
mod post;
mod template;

static DATABASE: OnceCell<RwLock<Database>> = OnceCell::new();

pub struct Database {
    pub repo: Repo,
    pub template: Template,
    pub posts: Posts,
    pub pages: Pages,
    pub authors: Authors,
}

impl Database {
    pub async fn init() -> Result<Database> {
        let mut repo = Repo::init()?;

        let ParsedGitRepo {
            template,
            pages_git_info,
            posts_git_info,
        } = repo.parse_repo().await?;

        let pages = Pages::from_git_file_info(pages_git_info, repo.tempdir.path()).await?;
        let posts = Posts::from_git_file_info(posts_git_info, repo.tempdir.path()).await?;
        let authors = Authors::generate(&pages, &posts);

        Ok(Self {
            repo,
            template,
            posts,
            pages,
            authors,
        })
    }

    pub async fn update(&mut self) -> Result<()> {
        let ParsedGitRepo {
            template,
            pages_git_info,
            posts_git_info,
        } = self.repo.parse_repo().await?;

        RouteTable::clear().await;

        self.template = template;
        self.pages = Pages::from_git_file_info(pages_git_info, self.repo.tempdir.path()).await?;
        self.posts = Posts::from_git_file_info(posts_git_info, self.repo.tempdir.path()).await?;
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
    Free {
        from: DateTime<Tz>,
        to: DateTime<Tz>,
    },
}

impl TimeRange {
    pub fn from_year_month(year: &str, month: Option<&str>) -> Option<Self> {
        let tz = &Config::read().application.timezone;

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

    pub fn from_timestamps(from: i64, to: i64) -> Option<Self> {
        if from > to {
            return None;
        }

        let tz = &Config::read().application.timezone;

        let from = tz.timestamp_opt(from, 0).single()?;
        let to = tz.timestamp_opt(to, 0).single()?;

        Some(Self::Free { from, to })
    }

    pub fn from(&self) -> &DateTime<Tz> {
        match self {
            Self::Year { from, .. } => from,
            Self::Month { from, .. } => from,
            Self::Free { from, .. } => from,
        }
    }

    pub fn to(&self) -> &DateTime<Tz> {
        match self {
            Self::Year { to, .. } => to,
            Self::Month { to, .. } => to,
            Self::Free { to, .. } => to,
        }
    }
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Year { year, .. } => write!(f, "Year: {}", year),
            Self::Month { year, month, .. } => write!(f, "Year: {} Month: {:02}", year, month),
            Self::Free { from, to } => write!(f, "From {} To {}", from, to),
        }
    }
}

pub struct ListInfo {
    pub current_page_num_in_list: usize,
    pub total_page: usize,
    pub total_num_of_articles_in_list: usize,
    pub page_num_pos_in_url_start_idx: usize,
    pub page_num_pos_in_url_end_idx: usize,
    pub is_page_num_the_first_param_in_query: bool,
}

impl ListInfo {
    pub fn new(
        current_page_num_in_list: usize,
        total_num_of_articles_in_list: usize,
        page_num_pos_in_url: (usize, usize),
        is_page_num_the_first_param_in_query: bool,
    ) -> Self {
        Self {
            current_page_num_in_list,
            total_page: total_num_of_articles_in_list / Config::read().site.list_posts_count + 1,
            total_num_of_articles_in_list,
            page_num_pos_in_url_start_idx: page_num_pos_in_url.0,
            page_num_pos_in_url_end_idx: page_num_pos_in_url.0,
            is_page_num_the_first_param_in_query,
        }
    }

    pub fn param_key(&self) -> &str {
        if self.is_page_num_the_first_param_in_query {
            "?page="
        } else {
            "&page="
        }
    }
}
