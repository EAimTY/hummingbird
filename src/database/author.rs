use std::collections::{hash_map::Entry, HashMap};

use super::{Pages, Posts};

#[derive(Debug, Clone)]
pub struct Authors {
    pub authors: Vec<String>,
    pub author_map: HashMap<String, AuthorInfo>,
}

impl Authors {
    pub fn new() -> Self {
        Self {
            authors: Vec::new(),
            author_map: HashMap::new(),
        }
    }

    pub fn get_author_id(&mut self, author: Option<&str>) -> Option<usize> {
        if let Some(author) = author {
            match self.author_map.entry(author.to_owned()) {
                Entry::Occupied(entry) => Some(entry.get().idx),
                Entry::Vacant(entry) => {
                    self.authors.push(author.to_owned());

                    let idx = self.authors.len() - 1;
                    entry.insert(AuthorInfo::new(idx));

                    Some(idx)
                }
            }
        } else {
            None
        }
    }

    pub fn update_index(&mut self, pages: &Pages, posts: &Posts) {
        pages
            .data
            .iter()
            .enumerate()
            .filter(|(_, page)| page.author_id.is_some())
            .for_each(|(idx, page)| {
                let author_info = self
                    .author_map
                    .get_mut(&self.authors[page.author_id.unwrap()])
                    .unwrap();
                author_info.pages.push(idx);
            });

        posts
            .data
            .iter()
            .enumerate()
            .filter(|(_, post)| post.author_id.is_some())
            .for_each(|(idx, post)| {
                let author_info = self
                    .author_map
                    .get_mut(&self.authors[post.author_id.unwrap()])
                    .unwrap();
                author_info.posts.push(idx);
            });
    }
}

#[derive(Debug, Clone)]
pub struct AuthorInfo {
    pub idx: usize,
    pub posts: Vec<usize>,
    pub pages: Vec<usize>,
}

impl AuthorInfo {
    pub fn new(idx: usize) -> Self {
        Self {
            idx,
            posts: Vec::new(),
            pages: Vec::new(),
        }
    }
}
