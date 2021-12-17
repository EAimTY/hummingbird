use super::{markdown, parameter::*};
use crate::{
    database::{Database, ListInfo, Page, Post, PostFilter, TimeRange},
    Config,
};
use chrono::DateTime;
use chrono_tz::Tz;
use hyper::{Body, Request, Uri};
use std::borrow::Cow;

pub struct SiteDataMap<'d> {
    url: Cow<'d, str>,
    name: Cow<'d, str>,
    description: Cow<'d, str>,
    page_list: Cow<'d, str>,
    recent_posts: Cow<'d, str>,
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
            url: Cow::Borrowed(&Config::read().site.url),
            name: Cow::Borrowed(&Config::read().site.name),
            description: Cow::Borrowed(Config::read().site.description.as_deref().unwrap_or("")),
            page_list: Cow::Owned(page_list),
            recent_posts: Cow::Owned(recent_posts),
        }
    }

    pub fn get(&'d self, param: &SiteParameter) -> Cow<'d, str> {
        match param {
            SiteParameter::Url => Cow::Borrowed(&self.url),
            SiteParameter::Name => Cow::Borrowed(&self.name),
            SiteParameter::Description => Cow::Borrowed(&self.description),
            SiteParameter::PageList => Cow::Borrowed(&self.page_list),
            SiteParameter::RecentPosts => Cow::Borrowed(&self.recent_posts),
        }
    }
}

pub struct DocumentDataMap<'d> {
    title: Cow<'d, str>,
    url: &'d Uri,
    breadcrumbs: Cow<'d, str>,
    page_nav: Cow<'d, str>,
    current_page_num_in_list: usize,
    total_num_of_articles_in_list: usize,
}

impl<'d> DocumentDataMap<'d> {
    const EMPTY_PAGE_NAV: &'static str = "";

    pub fn from_page(req: &'d Request<Body>, page: &'d Page) -> Self {
        Self {
            title: Cow::Borrowed(&page.title),
            url: req.uri(),
            breadcrumbs: Cow::Owned(format!("<span>Page: {}</span>", page.title)),
            page_nav: Cow::Borrowed(Self::EMPTY_PAGE_NAV),
            current_page_num_in_list: 0,
            total_num_of_articles_in_list: 0,
        }
    }

    pub fn from_post(req: &'d Request<Body>, post: &'d Post) -> Self {
        Self {
            title: Cow::Borrowed(&post.title),
            url: req.uri(),
            breadcrumbs: Cow::Owned(format!("<span>Post: {}</span>", post.title)),
            page_nav: Cow::Borrowed(Self::EMPTY_PAGE_NAV),
            current_page_num_in_list: 0,
            total_num_of_articles_in_list: 0,
        }
    }

    pub fn from_index(req: &'d Request<Body>, list_info: ListInfo) -> Self {
        Self {
            title: Cow::Borrowed(""),
            url: req.uri(),
            breadcrumbs: Cow::Borrowed("<span></span>"),
            page_nav: Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
            current_page_num_in_list: list_info.current_page_num_in_list,
            total_num_of_articles_in_list: list_info.total_num_of_articles_in_list,
        }
    }

    pub fn from_search(
        req: &'d Request<Body>,
        filters: Vec<PostFilter>,
        list_info: ListInfo,
    ) -> Self {
        let mut breadcrumbs = String::new();
        filters.iter().for_each(|filter| {
            breadcrumbs.push_str(r#"<span>"#);
            let (breadcrumb_type, breadcrumb_value) = filter.to_breadcrumb();
            breadcrumbs.push_str(breadcrumb_type);
            breadcrumbs.push_str(r#": "#);
            breadcrumbs.push_str(&breadcrumb_value);
            breadcrumbs.push_str(r#"</span>"#);
        });

        Self {
            title: Cow::Borrowed("Search"),
            url: req.uri(),
            breadcrumbs: Cow::Owned(breadcrumbs),
            page_nav: Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
            current_page_num_in_list: list_info.current_page_num_in_list,
            total_num_of_articles_in_list: list_info.total_num_of_articles_in_list,
        }
    }

    pub fn from_author(req: &'d Request<Body>, author: &'d str, list_info: ListInfo) -> Self {
        Self {
            title: Cow::Owned(format!("Author: {}", author)),
            url: req.uri(),
            breadcrumbs: Cow::Owned(format!("<span>Author: {}</span>", author)),
            page_nav: Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
            current_page_num_in_list: list_info.current_page_num_in_list,
            total_num_of_articles_in_list: list_info.total_num_of_articles_in_list,
        }
    }

    pub fn from_time_range(
        req: &'d Request<Body>,
        time_range: &'d TimeRange,
        list_info: ListInfo,
    ) -> Self {
        let breadcrumbs = format!("<span>{}</span>", time_range);

        Self {
            title: Cow::Owned(time_range.to_string()),
            url: req.uri(),
            breadcrumbs: Cow::Owned(breadcrumbs),
            page_nav: Cow::Owned(Self::gen_page_nav(req.uri(), &list_info)),
            current_page_num_in_list: list_info.current_page_num_in_list,
            total_num_of_articles_in_list: list_info.total_num_of_articles_in_list,
        }
    }

    pub fn from_not_found(req: &'d Request<Body>) -> Self {
        Self {
            title: Cow::Borrowed("Not Found"),
            url: req.uri(),
            breadcrumbs: Cow::Borrowed("<span>Not Found</span>"),
            page_nav: Cow::Borrowed(Self::EMPTY_PAGE_NAV),
            current_page_num_in_list: 0,
            total_num_of_articles_in_list: 0,
        }
    }

    pub fn get(&'d self, param: &DocumentParameter) -> Cow<'d, str> {
        match param {
            DocumentParameter::Title => Cow::Borrowed(&self.title),
            DocumentParameter::Url => Cow::Owned(self.url.to_string()),
            DocumentParameter::Breadcrumb => Cow::Borrowed(&self.breadcrumbs),
            DocumentParameter::PageNav => Cow::Borrowed(&self.page_nav),
            DocumentParameter::CurrentPageNumInList => {
                Cow::Owned(self.current_page_num_in_list.to_string())
            }
            DocumentParameter::TotalNumOfArticleInList => {
                Cow::Owned(self.total_num_of_articles_in_list.to_string())
            }
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
    title: Cow<'d, str>,
    url: Cow<'d, str>,
    content: Cow<'d, str>,
    author: Cow<'d, str>,
    create_time: &'d DateTime<Tz>,
    modify_time: &'d DateTime<Tz>,
}

impl<'d> PageDataMap<'d> {
    pub fn from_page(page: &'d Page) -> Self {
        Self {
            title: Cow::Borrowed(&page.title),
            url: Cow::Borrowed(&page.url),
            content: Cow::Owned(markdown::md_to_html(&page.content)),
            author: Cow::Borrowed(page.author.as_deref().unwrap_or("Anonymous")),
            create_time: &page.create_time,
            modify_time: &page.modify_time,
        }
    }

    pub fn get(&'d self, param: &PageParameter) -> Cow<'d, str> {
        match param {
            PageParameter::Title => Cow::Borrowed(&self.title),
            PageParameter::Url => Cow::Borrowed(&self.url),
            PageParameter::Content => Cow::Borrowed(&self.content),
            PageParameter::Author => Cow::Borrowed(&self.author),
            PageParameter::CreateTime => Cow::Owned(self.create_time.to_string()),
            PageParameter::ModifyTime => Cow::Owned(self.modify_time.to_string()),
        }
    }
}

pub struct PostDataMap<'d> {
    title: Cow<'d, str>,
    url: Cow<'d, str>,
    content: Cow<'d, str>,
    author: Cow<'d, str>,
    create_time: &'d DateTime<Tz>,
    modify_time: &'d DateTime<Tz>,
}

impl<'d> PostDataMap<'d> {
    pub fn from_post(post: &'d Post) -> Self {
        Self {
            title: Cow::Borrowed(&post.title),
            url: Cow::Borrowed(&post.url),
            content: Cow::Owned(markdown::md_to_html(&post.content)),
            author: Cow::Borrowed(post.author.as_deref().unwrap_or("Anonymous")),
            create_time: &post.create_time,
            modify_time: &post.modify_time,
        }
    }

    pub fn get(&'d self, param: &PostParameter) -> Cow<'d, str> {
        match param {
            PostParameter::Title => Cow::Borrowed(&self.title),
            PostParameter::Url => Cow::Borrowed(&self.url),
            PostParameter::Content => Cow::Borrowed(&self.content),
            PostParameter::Author => Cow::Borrowed(&self.author),
            PostParameter::CreateTime => Cow::Owned(self.create_time.to_string()),
            PostParameter::ModifyTime => Cow::Owned(self.modify_time.to_string()),
        }
    }
}

pub struct SummaryDataMap<'d> {
    title: Cow<'d, str>,
    url: Cow<'d, str>,
    summary: Cow<'d, str>,
    author: Cow<'d, str>,
    create_time: &'d DateTime<Tz>,
    modify_time: &'d DateTime<Tz>,
}

impl<'d> SummaryDataMap<'d> {
    pub fn from_post(post: &'d Post) -> Self {
        let more_indicator_idx = post
            .content
            .find("<!--more-->")
            .unwrap_or(post.content.len());
        let post_summary = &post.content[..more_indicator_idx].trim_end();

        Self {
            title: Cow::Borrowed(&post.title),
            url: Cow::Borrowed(&post.url),
            summary: Cow::Owned(markdown::md_to_html(post_summary)),
            author: Cow::Borrowed(post.author.as_deref().unwrap_or("Anonymous")),
            create_time: &post.create_time,
            modify_time: &post.modify_time,
        }
    }

    pub fn get(&'d self, param: &SummaryParameter) -> Cow<'d, str> {
        match param {
            SummaryParameter::Title => Cow::Borrowed(&self.title),
            SummaryParameter::Url => Cow::Borrowed(&self.url),
            SummaryParameter::Summary => Cow::Borrowed(&self.summary),
            SummaryParameter::Author => Cow::Borrowed(&self.author),
            SummaryParameter::CreateTime => Cow::Owned(self.create_time.to_string()),
            SummaryParameter::ModifyTime => Cow::Owned(self.modify_time.to_string()),
        }
    }
}
