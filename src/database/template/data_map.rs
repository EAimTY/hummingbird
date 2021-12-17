use super::{markdown, parameter::*};
use crate::{
    database::{Database, ListInfo, Page, Post, TimeRange},
    Config,
};
use hyper::{Body, Request, Uri};
use std::borrow::Cow;

pub struct SiteDataMap<'d> {
    data: (
        Cow<'d, str>,
        Cow<'d, str>,
        Cow<'d, str>,
        Cow<'d, str>,
        Cow<'d, str>,
    ),
}

impl<'d> SiteDataMap<'d> {
    pub fn from_config_and_db(db: &Database) -> Self {
        let mut page_list = String::from(r#"<ol id="page_list">"#);
        db.pages.data.iter().for_each(|page| {
            page_list.push_str(r#"<li><a herf=""#);
            page_list.push_str(&page.url);
            page_list.push_str(r#"">"#);
            page_list.push_str(&page.title);
            page_list.push_str(r#"</a></li>"#);
        });
        page_list.push_str(r#"</ol>"#);

        let mut recent_posts = String::from(r#"<ol id="recent_posts">"#);
        db.posts
            .data
            .iter()
            .rev()
            .take(Config::read().site.list_posts_count)
            .for_each(|post| {
                recent_posts.push_str(r#"<li><a herf=""#);
                recent_posts.push_str(&post.url);
                recent_posts.push_str(r#"">"#);
                recent_posts.push_str(&post.title);
                recent_posts.push_str(r#"</a></li>"#);
            });
        recent_posts.push_str(r#"</ol>"#);

        Self {
            data: (
                Cow::Borrowed(&Config::read().site.url),
                Cow::Borrowed(&Config::read().site.name),
                Cow::Borrowed(Config::read().site.description.as_deref().unwrap_or("")),
                Cow::Owned(page_list),
                Cow::Owned(recent_posts),
            ),
        }
    }

    pub fn get(&'d self, param: &SiteParameter) -> Cow<'d, str> {
        match param {
            SiteParameter::Url => Cow::Borrowed(&self.data.0),
            SiteParameter::Name => Cow::Borrowed(&self.data.1),
            SiteParameter::Description => Cow::Borrowed(&self.data.2),
            SiteParameter::PageList => Cow::Borrowed(&self.data.3),
            SiteParameter::RecentPosts => Cow::Borrowed(&self.data.4),
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

    pub fn from_index(req: &'d Request<Body>, list_info: ListInfo) -> Self {
        Self {
            data: (
                Cow::Borrowed("Index"),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
                list_info.current_page_num_in_list,
                list_info.total_num_of_articles_in_list,
            ),
        }
    }

    pub fn from_search(req: &'d Request<Body>, list_info: ListInfo) -> Self {
        Self {
            data: (
                Cow::Borrowed("Search"),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
                list_info.current_page_num_in_list,
                list_info.total_num_of_articles_in_list,
            ),
        }
    }

    pub fn from_author(req: &'d Request<Body>, author: &'d str, list_info: ListInfo) -> Self {
        Self {
            data: (
                Cow::Borrowed(author),
                req.uri(),
                Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
                list_info.current_page_num_in_list,
                list_info.total_num_of_articles_in_list,
            ),
        }
    }

    pub fn from_time_range(
        req: &'d Request<Body>,
        time_range: &'d TimeRange,
        list_info: ListInfo,
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
                Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
                list_info.current_page_num_in_list,
                list_info.total_num_of_articles_in_list,
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
            DocumentParameter::CurrentPageNumInList => Cow::Owned(self.data.3.to_string()),
            DocumentParameter::TotalNumOfArticleInList => Cow::Owned(self.data.4.to_string()),
        }
    }

    fn gen_page_nav(url: &Uri, list_info: &ListInfo) -> String {
        let url = url.to_string();
        let url_part_front = &url[..list_info.page_num_pos_in_url_start_idx];
        let url_part_back = &url[list_info.page_num_pos_in_url_end_idx..];

        let mut result = String::from(r#"<ol id="page_nav">"#);

        if list_info.current_page_num_in_list != 1 {
            result.push_str(r#"<li id="prev"><a herf=""#);
            result.push_str(url_part_front);
            result.push_str(list_info.param_key());
            result.push_str(&(list_info.current_page_num_in_list - 1).to_string());
            result.push_str(url_part_back);
            result.push_str(r#"">prev</a></li>"#);
        }

        for page_num in 1..list_info.current_page_num_in_list {
            result.push_str(r#"<li id="current"><a herf=""#);
            result.push_str(url_part_front);
            result.push_str(list_info.param_key());
            result.push_str(&page_num.to_string());
            result.push_str(url_part_back);
            result.push_str(r#"">"#);
            result.push_str(&page_num.to_string());
            result.push_str(r#"</a></li>"#);
        }

        result.push_str(r#"<li id="current"><a herf=""#);
        result.push_str(&url);
        result.push_str(r#"">"#);
        result.push_str(&(list_info.current_page_num_in_list).to_string());
        result.push_str(r#"</a></li>"#);

        if list_info.current_page_num_in_list != list_info.total_page {
            for page_num in list_info.current_page_num_in_list + 1..=list_info.total_page {
                result.push_str(r#"<li id="current"><a herf=""#);
                result.push_str(url_part_front);
                result.push_str(list_info.param_key());
                result.push_str(&page_num.to_string());
                result.push_str(url_part_back);
                result.push_str(r#"">"#);
                result.push_str(&page_num.to_string());
                result.push_str(r#"</a></li>"#);
            }

            result.push_str(r#"<li id="next"><a herf=""#);
            result.push_str(url_part_front);
            result.push_str(list_info.param_key());
            result.push_str(&(list_info.current_page_num_in_list + 1).to_string());
            result.push_str(url_part_back);
            result.push_str(r#"">next</a></li>"#);
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
        let more_indicator_idx = post
            .content
            .find("<!--more-->")
            .unwrap_or(post.content.len());
        let post_summary = &post.content[..more_indicator_idx].trim_end();

        Self {
            data: (
                Cow::Borrowed(&post.title),
                Cow::Borrowed(&post.url),
                Cow::Owned(markdown::md_to_html(post_summary)),
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
