use super::git::GitFileInfo;
use crate::{Config, RouteTable};
use anyhow::Result;
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
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
    pub data: Vec<Page>,
}

impl Pages {
    pub async fn from_git_file_info(
        file_info: HashMap<PathBuf, GitFileInfo>,
        tempdir: &Path,
    ) -> Result<Self> {
        let mut data = BinaryHeap::new();
        let page_url_regex_args = Regex::new(r":slug").unwrap();

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

        let path_map = data
            .iter()
            .enumerate()
            .map(|(idx, page)| (page.path.to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        RouteTable::update_page_map(path_map).await;

        Ok(Self { data })
    }

    pub fn get(&self, id: usize) -> &Page {
        &self.data[id]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    pub path: String,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub create_time: DateTime<Tz>,
    pub modify_time: DateTime<Tz>,
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
        let tz = &Config::read().settings.timezone;
        let create_time = tz.timestamp(create_time, 0);
        let modify_time = tz.timestamp(modify_time, 0);

        let path = url_regex_args
            .replace_all(
                &Config::read().url_patterns.page_url,
                |cap: &Captures| match &cap[0] {
                    ":slug" => &title,
                    _ => unreachable!(),
                },
            )
            .into_owned();

        Self {
            path,
            title,
            content,
            author,
            create_time,
            modify_time,
        }
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
