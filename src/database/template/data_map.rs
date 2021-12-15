use super::parameter::*;
use crate::{
    database::{Page, Post},
    Config,
};
use std::borrow::Cow;

pub struct SiteDataMap<'d> {
    data: [Cow<'d, str>; 1],
}

impl SiteDataMap<'_> {
    pub fn from_config() -> Self {
        Self {
            data: [Cow::Borrowed(&Config::read().site.name)],
        }
    }

    pub fn get(&self, param: &SiteParameter) -> &str {
        match param {
            SiteParameter::Name => &self.data[0],
        }
    }
}

pub struct DocumentDataMap<'d> {
    data: [Cow<'d, str>; 1],
}

impl<'d> DocumentDataMap<'d> {
    pub fn from_page(page: &'d Page) -> Self {
        Self {
            data: [Cow::Borrowed(&page.title)],
        }
    }

    pub fn from_post(post: &'d Post) -> Self {
        Self {
            data: [Cow::Borrowed(&post.title)],
        }
    }

    pub fn from_index() -> Self {
        Self {
            data: [Cow::Borrowed("Index")],
        }
    }

    pub fn from_search() -> Self {
        Self {
            data: [Cow::Borrowed("Search")],
        }
    }

    pub fn from_author(author: &'d str) -> Self {
        Self {
            data: [Cow::Borrowed(author)],
        }
    }

    pub fn from_time_range(time_range: &'d str) -> Self {
        Self {
            data: [Cow::Borrowed(time_range)],
        }
    }

    pub fn get(&self, param: &DocumentParameter) -> &str {
        match param {
            DocumentParameter::Title => &self.data[0],
        }
    }
}

pub struct PageDataMap<'d> {
    data: [Cow<'d, str>; 3],
}

impl<'d> PageDataMap<'d> {
    pub fn from_page(page: &'d Page) -> Self {
        Self {
            data: [
                Cow::Borrowed(&page.title),
                Cow::Borrowed(&page.path),
                Cow::Borrowed(&page.content),
            ],
        }
    }

    pub fn get(&self, param: &PageParameter) -> &str {
        match param {
            PageParameter::Title => &self.data[0],
            PageParameter::Link => &self.data[1],
            PageParameter::Content => &self.data[2],
        }
    }
}

pub struct PostDataMap<'d> {
    data: [Cow<'d, str>; 3],
}

impl<'d> PostDataMap<'d> {
    pub fn from_post(post: &'d Post) -> Self {
        Self {
            data: [
                Cow::Borrowed(&post.title),
                Cow::Borrowed(&post.path),
                Cow::Borrowed(&post.content),
            ],
        }
    }

    pub fn get(&self, param: &PostParameter) -> &str {
        match param {
            PostParameter::Title => &self.data[0],
            PostParameter::Link => &self.data[1],
            PostParameter::Content => &self.data[2],
        }
    }
}

pub struct SummaryDataMap<'d> {
    data: [Cow<'d, str>; 3],
}

impl<'d> SummaryDataMap<'d> {
    pub fn from_post(post: &'d Post) -> Self {
        Self {
            data: [
                Cow::Borrowed(&post.title),
                Cow::Borrowed(&post.path),
                Cow::Borrowed(&post.content),
            ],
        }
    }

    pub fn from_page(page: &'d Page) -> Self {
        Self {
            data: [
                Cow::Borrowed(&page.title),
                Cow::Borrowed(&page.path),
                Cow::Borrowed(&page.content),
            ],
        }
    }

    pub fn get(&self, param: &SummaryParameter) -> &str {
        match param {
            SummaryParameter::Title => &self.data[0],
            SummaryParameter::Link => &self.data[1],
            SummaryParameter::Content => &self.data[2],
        }
    }
}
