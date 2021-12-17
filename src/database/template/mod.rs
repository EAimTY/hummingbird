use self::{data_map::*, parameter::*};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{borrow::Cow, path::Path};
use tokio::fs;

pub mod data_map;
mod markdown;
mod parameter;
mod render;

#[derive(Clone, Debug)]
pub struct Template {
    header: Vec<Part>,
    footer: Vec<Part>,
    page_nav: Vec<Part>,
    page: Vec<Part>,
    post: Vec<Part>,
    summary: Vec<Part>,
    not_found: Vec<Part>,
}

impl Template {
    pub async fn from_directory(path: &Path) -> Result<Self> {
        let param_pattern = Regex::new(r"\{:[a-z._]+\}").unwrap();

        let header = fs::read_to_string(path.join("header.html")).await?;
        let header = Self::parse_string(&header, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            "{:document.current_page_num_in_list}" => {
                Ok(Part::Document(DocumentParameter::CurrentPageNumInList))
            }
            "{:document.total_num_of_articles_in_list}" => {
                Ok(Part::Document(DocumentParameter::TotalNumOfArticleInList))
            }
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let footer = fs::read_to_string(path.join("footer.html")).await?;
        let footer = Self::parse_string(&footer, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            "{:document.current_page_num_in_list}" => {
                Ok(Part::Document(DocumentParameter::CurrentPageNumInList))
            }
            "{:document.total_num_of_articles_in_list}" => {
                Ok(Part::Document(DocumentParameter::TotalNumOfArticleInList))
            }
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let page_nav = fs::read_to_string(path.join("page_nav.html")).await?;
        let page_nav = Self::parse_string(&page_nav, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            "{:document.page_nav}" => Ok(Part::Document(DocumentParameter::PageNav)),
            "{:document.current_page_num_in_list}" => {
                Ok(Part::Document(DocumentParameter::CurrentPageNumInList))
            }
            "{:document.total_num_of_articles_in_list}" => {
                Ok(Part::Document(DocumentParameter::TotalNumOfArticleInList))
            }
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let page = fs::read_to_string(path.join("page.html")).await?;
        let page = Self::parse_string(&page, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            "{:page.title}" => Ok(Part::Page(PageParameter::Title)),
            "{:page.link}" => Ok(Part::Page(PageParameter::Url)),
            "{:page.content}" => Ok(Part::Page(PageParameter::Content)),
            "{:page.author}" => Ok(Part::Page(PageParameter::Author)),
            "{:page.create_time}" => Ok(Part::Page(PageParameter::CreateTime)),
            "{:page.modify_time}" => Ok(Part::Page(PageParameter::ModifyTime)),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let post = fs::read_to_string(path.join("post.html")).await?;
        let post = Self::parse_string(&post, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            "{:post.title}" => Ok(Part::Post(PostParameter::Title)),
            "{:post.link}" => Ok(Part::Post(PostParameter::Url)),
            "{:post.content}" => Ok(Part::Post(PostParameter::Content)),
            "{:post.author}" => Ok(Part::Post(PostParameter::Author)),
            "{:post.create_time}" => Ok(Part::Post(PostParameter::CreateTime)),
            "{:post.modify_time}" => Ok(Part::Post(PostParameter::ModifyTime)),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let summary = fs::read_to_string(path.join("summary.html")).await?;
        let summary = Self::parse_string(&summary, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            "{:summary.title}" => Ok(Part::Summary(SummaryParameter::Title)),
            "{:summary.link}" => Ok(Part::Summary(SummaryParameter::Url)),
            "{:summary.content}" => Ok(Part::Summary(SummaryParameter::Summary)),
            "{:summary.author}" => Ok(Part::Summary(SummaryParameter::Author)),
            "{:summary.create_time}" => Ok(Part::Summary(SummaryParameter::CreateTime)),
            "{:summary.modify_time}" => Ok(Part::Summary(SummaryParameter::ModifyTime)),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let not_found = fs::read_to_string(path.join("not_found.html")).await?;
        let not_found = Self::parse_string(&not_found, &param_pattern, |str| match str {
            "{:site.url}" => Ok(Part::Site(SiteParameter::Url)),
            "{:site.name}" => Ok(Part::Site(SiteParameter::Name)),
            "{:site.description}" => Ok(Part::Site(SiteParameter::Description)),
            "{:site.page_list}" => Ok(Part::Site(SiteParameter::PageList)),
            "{:site.recent_posts}" => Ok(Part::Site(SiteParameter::RecentPosts)),
            "{:document.title}" => Ok(Part::Document(DocumentParameter::Title)),
            "{:document.url}" => Ok(Part::Document(DocumentParameter::Url)),
            "{:document.breadcrumbs}" => Ok(Part::Document(DocumentParameter::Breadcrumb)),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        Ok(Self {
            header,
            footer,
            page_nav,
            page,
            post,
            summary,
            not_found,
        })
    }

    fn parse_string<M>(str: &str, param_pattern: &Regex, param_matcher: M) -> Result<Vec<Part>>
    where
        M: Fn(&str) -> Result<Part>,
    {
        let mut result = Vec::new();
        let mut start = 0;

        for cap in param_pattern.find_iter(str) {
            result.push(Part::Static(str[start..cap.start()].to_owned()));
            start = cap.end();

            let param = param_matcher(cap.as_str())?;
            result.push(param);
        }

        result.push(Part::Static(str[start..].to_owned()));

        Ok(result)
    }

    fn header(&self, site_data: &SiteDataMap, document_data: &DocumentDataMap) -> String {
        self.header
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }

    fn footer(&self, site_data: &SiteDataMap, document_data: &DocumentDataMap) -> String {
        self.footer
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }

    fn page_nav(&self, site_data: &SiteDataMap, document_data: &DocumentDataMap) -> String {
        self.page_nav
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }

    fn page(
        &self,
        site_data: &SiteDataMap,
        document_data: &DocumentDataMap,
        page_data: &PageDataMap,
    ) -> String {
        self.page
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                Part::Page(param) => page_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }

    fn post(
        &self,
        site_data: &SiteDataMap,
        document_data: &DocumentDataMap,
        post_data: &PostDataMap,
    ) -> String {
        self.post
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                Part::Post(param) => post_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }

    fn summary(
        &self,
        site_data: &SiteDataMap,
        document_data: &DocumentDataMap,
        summary_data: &SummaryDataMap,
    ) -> String {
        self.summary
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                Part::Summary(param) => summary_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }

    fn not_found(&self, site_data: &SiteDataMap, document_data: &DocumentDataMap) -> String {
        self.not_found
            .iter()
            .map(|part| match part {
                Part::Static(str) => Cow::Borrowed(str.as_str()),
                Part::Site(param) => site_data.get(param),
                Part::Document(param) => document_data.get(param),
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub enum Part {
    Static(String),
    Site(SiteParameter),
    Document(DocumentParameter),
    Page(PageParameter),
    Post(PostParameter),
    Summary(SummaryParameter),
}
