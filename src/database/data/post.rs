use crate::Config;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use regex::{Captures, Regex};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
    url: String,
    title: String,
    content: String,
    author: String,
    author_email: Option<String>,
    create_time: DateTime<Utc>,
    modify_time: DateTime<Utc>,
}

impl Post {
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
        self.author_email.as_ref().map(|email| email.as_str())
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
