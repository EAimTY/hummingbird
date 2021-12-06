use super::{Pages, Posts};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Authors {
    pub authors: HashMap<String, AuthorInfo>,
}

impl Authors {
    pub fn generate(pages: &Pages, posts: &Posts) -> Self {
        let mut authors = HashMap::new();

        pages
            .data
            .iter()
            .enumerate()
            .filter(|(_, page)| page.author.is_some())
            .for_each(|(idx, page)| {
                let author = authors
                    .entry(page.author.as_ref().unwrap().clone())
                    .or_insert(AuthorInfo::new());
                author.pages_id.push(idx);
            });

        posts
            .data
            .iter()
            .enumerate()
            .filter(|(_, post)| post.author.is_some())
            .for_each(|(idx, post)| {
                let author = authors
                    .entry(post.author.as_ref().unwrap().clone())
                    .or_insert(AuthorInfo::new());
                author.posts_id.push(idx);
            });

        Self { authors }
    }
}

#[derive(Debug, Clone)]
pub struct AuthorInfo {
    pub posts_id: Vec<usize>,
    pub pages_id: Vec<usize>,
}

impl AuthorInfo {
    pub fn new() -> Self {
        Self {
            posts_id: Vec::new(),
            pages_id: Vec::new(),
        }
    }
}
