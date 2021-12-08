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
                    .or_insert_with(AuthorInfo::new);
                author.page_ids.push(idx);
            });

        posts
            .data
            .iter()
            .enumerate()
            .filter(|(_, post)| post.author.is_some())
            .for_each(|(idx, post)| {
                let author = authors
                    .entry(post.author.as_ref().unwrap().clone())
                    .or_insert_with(AuthorInfo::new);
                author.post_ids.push(idx);
            });

        Self { authors }
    }

    pub fn get_posts(&self, author: &str) -> Option<&[usize]> {
        self.authors
            .get(author)
            .map(|author| author.post_ids.as_slice())
    }
}

#[derive(Debug, Clone)]
pub struct AuthorInfo {
    pub post_ids: Vec<usize>,
    pub page_ids: Vec<usize>,
}

impl AuthorInfo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            post_ids: Vec::new(),
            page_ids: Vec::new(),
        }
    }
}
