use crate::config::Config;

use super::{Data, Database};
use std::{cmp::Ordering, collections::HashMap};

impl Database {
    pub async fn get_post(&self, path: &str) -> String {
        let database = self.data.read().await;

        let id = database.posts.url_map.get(path).unwrap();
        let post_data = &database.posts.data[*id];
        database.theme.render(Data::Post(post_data))
    }
}

#[derive(Debug)]
pub struct Posts {
    pub data: Vec<Post>,
    pub url_map: HashMap<String, usize>,
}

impl Posts {
    pub fn new() -> Self {
        Posts {
            data: Vec::new(),
            url_map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub create_time: i64,
    pub modify_time: i64,
}

impl Post {
    pub fn get_url(&self) -> String {
        let pattern = &Config::read().url_patterns.post_url;
        pattern.replace("{slug}", &self.title)
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
