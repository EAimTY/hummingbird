use anyhow::{bail, Result};
use regex::Regex;
use std::path::Path;
use tokio::fs;

mod render;

#[derive(Clone, Debug)]
pub struct Template {
    header: Vec<Part>,
    footer: Vec<Part>,
    page: Vec<Part>,
    post: Vec<Part>,
    summary: Vec<Part>,
}

impl Template {
    pub async fn from_directory(path: &Path) -> Result<Self> {
        let params = Regex::new(r"\{:[a-z.]+\}").unwrap();

        let header = {
            let header_str = fs::read_to_string(path.join("header.html")).await?;

            let mut header = Vec::new();
            let mut ptr = 0;
            for cap in params.find_iter(&header_str) {
                header.push(Part::Static(header_str[ptr..cap.start()].to_owned()));
                ptr = cap.end() + 1;
                match cap.as_str() {
                    "{:document.title}" => header.push(Part::DocumentTitle),
                    _ => bail!("Unknown parameter: {}", cap.as_str()),
                }
            }
            header
        };

        let footer = {
            let footer_str = fs::read_to_string(path.join("footer.html")).await?;

            let mut footer = Vec::new();
            let mut ptr = 0;
            for cap in params.find_iter(&footer_str) {
                footer.push(Part::Static(footer_str[ptr..cap.start()].to_owned()));
                ptr = cap.end() + 1;
                match cap.as_str() {
                    "{:document.title}" => footer.push(Part::DocumentTitle),
                    _ => bail!("Unknown parameter: {}", cap.as_str()),
                }
            }
            footer
        };

        let page = {
            let page_str = fs::read_to_string(path.join("page.html")).await?;

            let mut page = Vec::new();
            let mut ptr = 0;
            for cap in params.find_iter(&page_str) {
                page.push(Part::Static(page_str[ptr..cap.start()].to_owned()));
                ptr = cap.end() + 1;
                match cap.as_str() {
                    "{:page.title}" => page.push(Part::PageTitle),
                    "{:page.link}" => page.push(Part::PageLink),
                    "{:page.content}" => page.push(Part::PageContent),
                    _ => bail!("Unknown parameter: {}", cap.as_str()),
                }
            }
            page
        };

        let post = {
            let post_str = fs::read_to_string(path.join("post.html")).await?;

            let mut post = Vec::new();
            let mut ptr = 0;
            for cap in params.find_iter(&post_str) {
                post.push(Part::Static(post_str[ptr..cap.start()].to_owned()));
                ptr = cap.end() + 1;
                match cap.as_str() {
                    "{:post.title}" => post.push(Part::PostTitle),
                    "{:post.link}" => post.push(Part::PostLink),
                    "{:post.content}" => post.push(Part::PostContent),
                    _ => bail!("Unknown parameter: {}", cap.as_str()),
                }
            }
            post
        };

        let summary = {
            let summary_str = fs::read_to_string(path.join("summary.html")).await?;

            let mut summary = Vec::new();
            let mut ptr = 0;
            for cap in params.find_iter(&summary_str) {
                summary.push(Part::Static(summary_str[ptr..cap.start()].to_owned()));
                ptr = cap.end() + 1;
                match cap.as_str() {
                    "{:summary.title}" => summary.push(Part::SummaryTitle),
                    "{:summary.link}" => summary.push(Part::SummaryLink),
                    "{:summary.content}" => summary.push(Part::SummaryContent),
                    _ => bail!("Unknown parameter: {}", cap.as_str()),
                }
            }
            summary
        };

        Ok(Self {
            header,
            footer,
            page,
            post,
            summary,
        })
    }
}

#[derive(Clone, Debug)]
enum Part {
    Static(String),
    DocumentTitle,
    PageTitle,
    PageLink,
    PageContent,
    PostTitle,
    PostLink,
    PostContent,
    SummaryTitle,
    SummaryLink,
    SummaryContent,
}
