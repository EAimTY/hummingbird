use super::{markdown, parameter::*};
use crate::{
    database::{Page, Post, TimeRange},
    Config,
};
use hyper::{Body, Request, Uri};
use std::borrow::Cow;

pub struct SiteDataMap<'d> {
    data: (Cow<'d, str>,),
}

impl<'d> SiteDataMap<'d> {
    pub fn from_config() -> Self {
        Self {
            data: (Cow::Borrowed(&Config::read().site.name),),
        }
    }

    pub fn get(&'d self, param: &SiteParameter) -> Cow<'d, str> {
        match param {
            SiteParameter::Name => Cow::Borrowed(&self.data.0),
        }
    }
}

pub struct DocumentDataMap<'d> {
    data: (Cow<'d, str>, &'d Uri, Cow<'d, str>, usize, usize),
}

impl<'d> DocumentDataMap<'d> {
    const EMPTY_PAGE_NAV: &'static str = "";

    pub fn from_page(req: &'d Request<Body>, page: &'d Page) -> Self {
        Self {
            data: (
                Cow::Borrowed(&page.title),
                req.uri(),
                Cow::Borrowed(Self::EMPTY_PAGE_NAV),
                0,
                0,
            ),
        }
    }

    pub fn from_post(req: &'d Request<Body>, post: &'d Post) -> Self {
        Self {
            data: (
                Cow::Borrowed(&post.title),
                req.uri(),
                Cow::Borrowed(Self::EMPTY_PAGE_NAV),
                0,
                0,
            ),
        }
    }

    pub fn from_index(req: &'d Request<Body>, current_page: usize, total_page: usize) -> Self {
        Self {
            data: (
                Cow::Borrowed("Index"),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(current_page, total_page)),
                current_page,
                total_page,
            ),
        }
    }

    pub fn from_search(req: &'d Request<Body>, current_page: usize, total_page: usize) -> Self {
        Self {
            data: (
                Cow::Borrowed("Search"),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(current_page, total_page)),
                current_page,
                total_page,
            ),
        }
    }

    pub fn from_author(
        req: &'d Request<Body>,
        author: &'d str,
        current_page: usize,
        total_page: usize,
    ) -> Self {
        Self {
            data: (
                Cow::Borrowed(author),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(current_page, total_page)),
                current_page,
                total_page,
            ),
        }
    }

    pub fn from_time_range(
        req: &'d Request<Body>,
        time_range: &'d TimeRange,
        current_page: usize,
        total_page: usize,
    ) -> Self {
        let time_range = match time_range {
            TimeRange::Year { year, .. } => year.to_string(),
            TimeRange::Month { year, month, .. } => format!("{}-{}", year, month),
            TimeRange::Free { .. } => unreachable!(),
        };

        Self {
            data: (
                Cow::Owned(time_range),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(current_page, total_page)),
                current_page,
                total_page,
            ),
        }
    }

    pub fn from_not_found(req: &'d Request<Body>) -> Self {
        Self {
            data: (
                Cow::Borrowed("Not Found"),
                req.uri(),
                Cow::Borrowed(Self::EMPTY_PAGE_NAV),
                0,
                0,
            ),
        }
    }

    pub fn get(&'d self, param: &DocumentParameter) -> Cow<'d, str> {
        match param {
            DocumentParameter::Title => Cow::Borrowed(&self.data.0),
            DocumentParameter::Url => Cow::Owned(self.data.1.to_string()),
            DocumentParameter::PageNav => Cow::Borrowed(&self.data.2),
            DocumentParameter::CurrentPage => Cow::Owned(self.data.3.to_string()),
            DocumentParameter::TotalPage => Cow::Owned(self.data.4.to_string()),
        }
    }

    fn gen_page_nav(current_page: usize, total_page: usize) -> String {
        let mut result = String::from(r#"<ol id="page_nav">"#);

        if current_page != 1 {
            result.push_str(r#"<li id="prev"><a herf="">prev</a></li>"#);
        }

        if current_page != total_page {
            result.push_str(r#"<li id="next"><a herf="">next</a></li>"#);
        }

        result.push_str(r#"</ol>"#);

        result
    }
}

pub struct PageDataMap<'d> {
    data: (Cow<'d, str>, Cow<'d, str>, Cow<'d, str>),
}

impl<'d> PageDataMap<'d> {
    pub fn from_page(page: &'d Page) -> Self {
        Self {
            data: (
                Cow::Borrowed(&page.title),
                Cow::Borrowed(&page.url),
                Cow::Owned(markdown::md_to_html(&page.content)),
            ),
        }
    }

    pub fn get(&'d self, param: &PageParameter) -> Cow<'d, str> {
        match param {
            PageParameter::Title => Cow::Borrowed(&self.data.0),
            PageParameter::Url => Cow::Borrowed(&self.data.1),
            PageParameter::Content => Cow::Borrowed(&self.data.2),
        }
    }
}

pub struct PostDataMap<'d> {
    data: (Cow<'d, str>, Cow<'d, str>, Cow<'d, str>),
}

impl<'d> PostDataMap<'d> {
    pub fn from_post(post: &'d Post) -> Self {
        Self {
            data: (
                Cow::Borrowed(&post.title),
                Cow::Borrowed(&post.url),
                Cow::Owned(markdown::md_to_html(&post.content)),
            ),
        }
    }

    pub fn get(&'d self, param: &PostParameter) -> Cow<'d, str> {
        match param {
            PostParameter::Title => Cow::Borrowed(&self.data.0),
            PostParameter::Url => Cow::Borrowed(&self.data.1),
            PostParameter::Content => Cow::Borrowed(&self.data.2),
        }
    }
}

pub struct SummaryDataMap<'d> {
    data: (Cow<'d, str>, Cow<'d, str>, Cow<'d, str>),
}

impl<'d> SummaryDataMap<'d> {
    pub fn from_post(post: &'d Post) -> Self {
        Self {
            data: (
                Cow::Borrowed(&post.title),
                Cow::Borrowed(&post.url),
                Cow::Owned(markdown::md_to_html(&post.content)),
            ),
        }
    }

    pub fn get(&'d self, param: &SummaryParameter) -> Cow<'d, str> {
        match param {
            SummaryParameter::Title => Cow::Borrowed(&self.data.0),
            SummaryParameter::Url => Cow::Borrowed(&self.data.1),
            SummaryParameter::Content => Cow::Borrowed(&self.data.2),
        }
    }
}
