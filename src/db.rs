use crate::{config, op::Op, repo::Repo};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;

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

    pub async fn listen(&mut self, mut op_reciever: mpsc::Receiver<Op>) {
        while let Some(op) = op_reciever.recv().await {
            match op {
                Op::GetPost {
                    title,
                    channel_sender,
                } => {
                    if let Some(post) = self.posts.get(&title) {
                        channel_sender.send(post.content.clone());
                    }
                }
                Op::GetPage {
                    title,
                    channel_sender,
                } => {
                    if let Some(page) = self.pages.get(&title) {
                        channel_sender.send(page.content.clone());
                    }
                }
                Op::Update { channel_sender } => {
                    self.repo.fetch().unwrap();

                    self.posts = self.repo.parse_posts().unwrap();
                    self.pages = self.repo.parse_pages().unwrap();

                    channel_sender.send(String::from("done"));
                }
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
