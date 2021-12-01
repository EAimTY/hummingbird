use super::{git::GitFileInfo, DataType};
use crate::Config;
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
    data: Vec<Post>,
    url_map: HashMap<String, usize>,
    author_map: HashMap<String, (String, Vec<usize>)>,
}

impl Posts {
    pub async fn from_file_info(
        file_info: HashMap<PathBuf, GitFileInfo>,
        tempdir: &Path,
    ) -> Result<Self> {
        let mut data = BinaryHeap::new();
        let post_url_regex_args = Regex::new(r"(\{slug\}|\{year\}|\{month\})").unwrap();

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

        let url_map = data
            .iter()
            .enumerate()
            .map(|(idx, post)| (post.url().to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        let author_map =
            data.iter()
                .enumerate()
                .fold(HashMap::new(), |mut author_map, (idx, post)| {
                    let posts = author_map
                        .entry(post.author().to_owned())
                        .or_insert_with(Vec::new);
                    posts.push(idx);
                    author_map
                });

        let author_map = author_map
            .into_iter()
            .map(|(author, posts)| {
                (
                    Config::read()
                        .url_patterns
                        .author_url
                        .replace("{author}", &author.unwrap()),
                    (author.unwrap().to_owned(), posts),
                )
            })
            .collect();

        Ok(Self {
            data,
            url_map,
            author_map,
        })
    }

    pub fn get(&self, path: &str) -> Option<DataType> {
        self.url_map
            .get(path)
            .map(|id| DataType::Post(&self.data[*id]))
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

    pub fn get_author(&self, path: &str) -> Option<DataType> {
        if self.author_map.is_empty() {
            return None;
        }
        self.author_map.get(path).map(|(author, posts)| {
            let data = posts.iter().map(|id| &self.data[*id]).collect();
            DataType::Author {
                data,
                author: author.to_owned(),
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
    url: String,
    title: String,
    content: String,
    author: Option<String>,
    create_time: DateTime<Utc>,
    modify_time: DateTime<Utc>,
}

impl Post {
    pub fn new(
        title: String,
        content: String,
        author: Option<String>,
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

        let url = url_regex_args
            .replace_all(
                &Config::read().url_patterns.post_url,
                |cap: &Captures| match &cap[0] {
                    "{slug}" => &title,
                    "{year}" => &year,
                    "{month}" => &month,
                    _ => unreachable!(),
                },
            )
            .into_owned();

        Self {
            url,
            title,
            content,
            author,
            create_time,
            modify_time,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
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
