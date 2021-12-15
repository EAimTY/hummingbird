use self::params::Params;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::Path;
use tokio::fs;

mod params;
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
        let param_pattern = Regex::new(r"\{:[a-z.]+\}").unwrap();

        let header = fs::read_to_string(path.join("header.html")).await?;
        let header = Self::parse_string(&header, &param_pattern, &|str| match str {
            "{:site.name}" => Ok(Part::SiteName),
            "{:document.title}" => Ok(Part::DocumentTitle),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let footer = fs::read_to_string(path.join("footer.html")).await?;
        let footer = Self::parse_string(&footer, &param_pattern, &|str| match str {
            "{:site.name}" => Ok(Part::SiteName),
            "{:document.title}" => Ok(Part::DocumentTitle),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let page = fs::read_to_string(path.join("page.html")).await?;
        let page = Self::parse_string(&page, &param_pattern, &|str| match str {
            "{:site.name}" => Ok(Part::SiteName),
            "{:document.title}" => Ok(Part::DocumentTitle),
            "{:page.title}" => Ok(Part::PageTitle),
            "{:page.link}" => Ok(Part::PageLink),
            "{:page.content}" => Ok(Part::PageContent),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let post = fs::read_to_string(path.join("post.html")).await?;
        let post = Self::parse_string(&post, &param_pattern, &|str| match str {
            "{:site.name}" => Ok(Part::SiteName),
            "{:document.title}" => Ok(Part::DocumentTitle),
            "{:post.title}" => Ok(Part::PostTitle),
            "{:post.link}" => Ok(Part::PostLink),
            "{:post.content}" => Ok(Part::PostContent),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        let summary = fs::read_to_string(path.join("summary.html")).await?;
        let summary = Self::parse_string(&summary, &param_pattern, &|str| match str {
            "{:site.name}" => Ok(Part::SiteName),
            "{:document.title}" => Ok(Part::DocumentTitle),
            "{:summary.title}" => Ok(Part::SummaryTitle),
            "{:summary.link}" => Ok(Part::SummaryLink),
            "{:summary.content}" => Ok(Part::SummaryContent),
            _ => Err(anyhow!("Unknown parameter: {}", str)),
        })?;

        Ok(Self {
            header,
            footer,
            page,
            post,
            summary,
        })
    }

    fn parse_string(
        str: &str,
        param_pattern: &Regex,
        param_matcher: &dyn Fn(&str) -> Result<Part>,
    ) -> Result<Vec<Part>> {
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

    fn header(&self, params: &Params) -> String {
        self.header
            .iter()
            .map(|part| {
                if let Part::Static(str) = part {
                    str
                } else {
                    params.get(part)
                }
            })
            .collect()
    }

    fn footer(&self, params: &Params) -> String {
        self.footer
            .iter()
            .map(|part| {
                if let Part::Static(str) = part {
                    str
                } else {
                    params.get(part)
                }
            })
            .collect()
    }

    fn page(&self, params: &Params) -> String {
        self.page
            .iter()
            .map(|part| {
                if let Part::Static(str) = part {
                    str
                } else {
                    params.get(part)
                }
            })
            .collect()
    }

    fn post(&self, params: &Params) -> String {
        self.post
            .iter()
            .map(|part| {
                if let Part::Static(str) = part {
                    str
                } else {
                    params.get(part)
                }
            })
            .collect()
    }

    fn summary(&self, params: &Params) -> String {
        self.summary
            .iter()
            .map(|part| {
                if let Part::Static(str) = part {
                    str
                } else {
                    params.get(part)
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub enum Part {
    Static(String),
    SiteName,
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
