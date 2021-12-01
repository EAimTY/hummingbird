use super::{data_type::DataType, git::GitFileInfo};
use crate::Config;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::{Captures, Regex};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Debug)]
pub struct Pages {
    data: Vec<Page>,
    url_map: HashMap<String, usize>,
}

impl Pages {
    pub async fn from_file_info(
        file_info: HashMap<PathBuf, GitFileInfo>,
        tempdir: &Path,
    ) -> Result<Self> {
        let mut data = BinaryHeap::new();
        let page_url_regex_args = Regex::new(r"(\{slug\})").unwrap();

        for (path, info) in file_info.into_iter() {
            if path.extension() == Some(OsStr::new("md")) {
                let abs_path = tempdir.join(&path);

                let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let content = fs::read_to_string(abs_path).await?;

                let page = Page::new(
                    title,
                    content,
                    info.author,
                    info.create_time.unwrap(),
                    info.modify_time,
                    &page_url_regex_args,
                );
                data.push(page);
            }
        }

        let data = data.into_sorted_vec();

        let url_map = data
            .iter()
            .enumerate()
            .map(|(idx, page)| (page.url().to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        Ok(Self { data, url_map })
    }

    pub fn get(&self, path: &str) -> Option<DataType> {
        self.url_map
            .get(path)
            .map(|id| DataType::Page(&self.data[*id]))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    url: String,
    title: String,
    content: String,
    author: Option<String>,
    create_time: DateTime<Utc>,
    modify_time: DateTime<Utc>,
}

impl Page {
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

        let url = url_regex_args
            .replace_all(
                &Config::read().url_patterns.page_url,
                |cap: &Captures| match &cap[0] {
                    "{slug}" => &title,
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

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.create_time.cmp(&other.create_time) {
            Ordering::Equal => self.title.cmp(&other.title),
            other => other,
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
