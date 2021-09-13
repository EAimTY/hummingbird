use crate::{config, repo::Repo};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub enum DbOp {
    GetPost(String),
    GetPage(String),
}

pub struct Db<'a> {
    repo: Repo<'a>,
    posts: HashMap<String, Post>,
    pages: HashMap<String, Page>,
}

impl<'a> Db<'a> {
    pub fn init(config: &config::Config) -> Result<Self> {
        let mut repo = Repo::init(&config.git)?;
        repo.fetch()?;

        let posts = repo.parse_posts()?;
        let pages = repo.parse_pages()?;

        Ok(Self { repo, posts, pages })
    }

    pub async fn listen(&self, mut rx: mpsc::Receiver<DbOp>) {
        while let Some(op) = rx.recv().await {
            match op {
                DbOp::GetPost(title) => {
                    if let Some(post) = self.posts.get(&title) {
                        println!("{}", post.content);
                    }
                }
                DbOp::GetPage(title) => {}
            }
        }
    }
}

pub struct Post {
    pub content: String,
}

impl Post {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

pub struct Page {
    pub content: String,
}

impl Page {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
