use crate::Config;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::{Captures, Regex};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    url: String,
    title: String,
    content: String,
    author: String,
    author_email: Option<String>,
    create_time: DateTime<Utc>,
    modify_time: DateTime<Utc>,
}

impl Page {
    pub fn new(
        title: String,
        content: String,
        author: String,
        author_email: Option<String>,
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
            author_email,
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

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn author_email(&self) -> Option<&str> {
        self.author_email.as_deref()
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
