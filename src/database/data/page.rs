use crate::Config;
use chrono::{DateTime, Utc};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    pub title: String,
    pub content: String,
    pub author: String,
    pub author_email: Option<String>,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}

impl Page {
    pub fn get_url(&self) -> String {
        let pattern = &Config::read().url_patterns.page_url;
        pattern.replace("{slug}", &self.title)
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
