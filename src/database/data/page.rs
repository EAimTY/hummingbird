use crate::Config;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    pub title: String,
    pub content: String,
    pub author: String,
    pub author_email: Option<String>,
    pub create_time: i64,
    pub modify_time: i64,
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
