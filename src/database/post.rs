use super::{git::GitFileInfo, TimeRange};
use crate::{Config, RouteTable};
use anyhow::Result;
use chrono::{DateTime, Datelike, TimeZone};
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

        let site_url_len = Config::read().site.url.len();

        let map = data
            .iter()
            .enumerate()
            .map(|(idx, post)| (post.url[site_url_len..].to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        RouteTable::update_posts(map).await;

        Ok(Self { data })
    }

    pub fn get(&self, id: usize) -> &Post {
        &self.data[id]
    }

    pub fn get_multi(&self, id: &[usize], current_page: usize) -> Option<(Vec<&Post>, usize)> {
        Self::page_filter(id.iter().map(|id| &self.data[*id]), current_page)
    }

    pub fn get_time_range(
        &self,
        time_range: &TimeRange,
        current_page: usize,
    ) -> Option<(Vec<&Post>, usize)> {
        let from = self
            .data
            .partition_point(|post| &post.create_time < time_range.from());

        let to = self
            .data
            .partition_point(|post| &post.create_time <= time_range.to());

        Self::page_filter(self.data[from..to].iter(), current_page)
    }

    pub fn get_index(&self, current_page: usize) -> Option<(Vec<&Post>, usize)> {
        Self::page_filter(self.data.iter(), current_page)
    }

    pub fn search(
        &self,
        filters: &[PostFilter],
        current_page: usize,
    ) -> Option<(Vec<&Post>, usize)> {
        let mut posts: Box<dyn DoubleEndedIterator<Item = &Post>> = Box::new(self.data.iter());

        for filter in filters {
            posts = match filter {
                PostFilter::Keyword(keyword) => {
                    let filter = move |post: &&Post| {
                        post.title.contains(keyword) || post.content.contains(keyword)
                    };
                    Box::new(posts.filter(filter))
                }
                PostFilter::TimeRange(time_range) => {
                    let filter = move |post: &&Post| {
                        &post.create_time >= time_range.from()
                            && &post.create_time <= time_range.to()
                    };
                    Box::new(posts.filter(filter))
                }
                PostFilter::Author(author) => {
                    let filter = move |post: &&Post| match &post.author {
                        Some(post_author) => post_author == author,
                        None => false,
                    };
                    Box::new(posts.filter(filter))
                }
            }
        }

        Self::page_filter(posts, current_page)
    }

    fn page_filter<'p>(
        posts: impl DoubleEndedIterator<Item = &'p Post>,
        current_page: usize,
    ) -> Option<(Vec<&'p Post>, usize)> {
        let mut result = Vec::with_capacity(Config::read().site.list_posts_count);
        let mut total = 0;

        if Config::read().site.list_from_old_to_new {
            let mut posts_iter =
                posts.skip((current_page - 1) * Config::read().site.list_posts_count);

            while let Some(post) = posts_iter.next() {
                result.push(post);
                total += 1;

                if total == Config::read().site.list_posts_count {
                    break;
                }
            }

            total += (current_page - 1) * Config::read().site.list_posts_count + posts_iter.count();
        } else {
            let mut posts_iter = posts
                .rev()
                .skip((current_page - 1) * Config::read().site.list_posts_count);

            while let Some(post) = posts_iter.next() {
                result.push(post);
                total += 1;

                if total == Config::read().site.list_posts_count {
                    break;
                }
            }

            total += (current_page - 1) * Config::read().site.list_posts_count + posts_iter.count();
        }

        if !result.is_empty() {
            Some((result, total))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub url: String,
    pub content: String,
    pub author: Option<String>,
    pub create_time: DateTime<Tz>,
    pub modify_time: DateTime<Tz>,
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
        let tz = &Config::read().application.timezone;
        let create_time = tz.timestamp(create_time, 0);
        let modify_time = tz.timestamp(modify_time, 0);

        let year = create_time.year().to_string();

        let month = create_time.month().to_string();

        let path =
            url_regex_args.replace_all(&Config::read().url_patterns.post, |cap: &Captures| {
                match &cap[0] {
                    ":slug" => &title,
                    ":year" => &year,
                    ":month" => &month,
                    _ => unreachable!(),
                }
            });

        let url = format!("{}{}", &Config::read().site.url, path);

        Self {
            title,
            url,
            content,
            author,
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

pub enum PostFilter<'f> {
    Keyword(&'f str),
    TimeRange(TimeRange),
    Author(&'f str),
}

impl<'f> PostFilter<'f> {
    pub fn from_uri_query(query: &'f str) -> Option<Vec<Self>> {
        let filters = query
            .split('&')
            .filter_map(|part| {
                let (key, value) = part.split_once('=')?;

                match key {
                    "keyword" => Some(Self::Keyword(value)),
                    "time_range" => {
                        let (from, to) = part.split_once('-')?;
                        let (from, to) = (from.parse().ok()?, to.parse().ok()?);
                        Some(Self::TimeRange(TimeRange::from_timestamps(from, to)?))
                    }
                    "author" => Some(Self::Author(value)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if !filters.is_empty() {
            Some(filters)
        } else {
            None
        }
    }
}
