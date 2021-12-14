use anyhow::Result;
use std::path::Path;
use tokio::fs;

mod render;

#[derive(Clone, Debug)]
pub struct Template {
    header: String,
    footer: String,
    page: String,
    post: String,
    summary: String,
}

impl Template {
    pub async fn from_directory(path: &Path) -> Result<Self> {
        let header = fs::read_to_string(path.join("header.html")).await?;
        let footer = fs::read_to_string(path.join("footer.html")).await?;
        let page = fs::read_to_string(path.join("page.html")).await?;
        let post = fs::read_to_string(path.join("post.html")).await?;
        let summary = fs::read_to_string(path.join("summary.html")).await?;

        Ok(Self {
            header,
            footer,
            page,
            post,
            summary,
        })
    }
}
