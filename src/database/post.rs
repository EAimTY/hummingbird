use super::{git::GitFileInfo, DataType};
use crate::{Config, RouteTable};
use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use regex::{Captures, Regex};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Debug)]
pub struct Posts {
    pub data: Vec<Post>,
}

impl Posts {
    pub async fn from_git_file_info(
        file_info: HashMap<PathBuf, GitFileInfo>,
        tempdir: &Path,
    ) -> Result<Self> {
        let mut data = BinaryHeap::new();
        let post_url_regex_args = Regex::new(r":slug|:year|:month").unwrap();

        for (path, info) in file_info.into_iter() {
            if path.extension() == Some(OsStr::new("md")) {
                let abs_path = tempdir.join(&path);

                let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let content = fs::read_to_string(abs_path).await?;

                let post = Post::new(
                    title,
                    content,
                    info.author,
                    info.create_time.unwrap(),
                    info.modify_time,
                    &post_url_regex_args,
                );
                data.push(post);
            }
        }

        let data = data.into_sorted_vec();

        let path_map = data
            .iter()
            .enumerate()
            .map(|(idx, post)| (post.path.to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        RouteTable::update_post_map(path_map).await;

        Ok(Self { data })
    }

    pub fn get(&self, id: usize) -> Option<DataType> {
        self.data.get(id).map(|post| DataType::Post(post))
    }

    pub fn get_index(&self) -> Option<DataType> {
        if self.data.is_empty() {
            return None;
        }

        let data = if Config::read().settings.index_posts_from_old_to_new {
            self.data
                .iter()
                .take(Config::read().settings.index_posts_count)
                .collect()
        } else {
            self.data
                .iter()
                .rev()
                .take(Config::read().settings.index_posts_count)
                .collect()
        };

        Some(DataType::Index { data })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
    pub path: String,
    pub title: String,
    pub content: String,
    pub author_id: Option<usize>,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}

impl Post {
    pub fn new(
        title: String,
        content: String,
        author_id: Option<usize>,
        create_time: i64,
        modify_time: i64,
        url_regex_args: &Regex,
    ) -> Self {
        let create_time = DateTime::from_utc(NaiveDateTime::from_timestamp(create_time, 0), Utc);
        let modify_time = DateTime::from_utc(NaiveDateTime::from_timestamp(modify_time, 0), Utc);

        let year = create_time
            .with_timezone(&Config::read().settings.timezone)
            .year()
            .to_string();

        let month = create_time
            .with_timezone(&Config::read().settings.timezone)
            .month()
            .to_string();

        let path = url_regex_args
            .replace_all(
                &Config::read().url_patterns.post_url,
                |cap: &Captures| match &cap[0] {
                    ":slug" => &title,
                    ":year" => &year,
                    ":month" => &month,
                    _ => unreachable!(),
                },
            )
            .into_owned();

        Self {
            path,
            title,
            content,
            author_id,
            create_time,
            modify_time,
        }
    }
}

impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.create_time.cmp(&other.create_time) {
            Ordering::Equal => self.title.cmp(&other.title),
            other => other,
        }
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
