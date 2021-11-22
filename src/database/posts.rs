use crate::data::{List, Post};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Posts {
    data: Vec<Post>,
    url_map: HashMap<String, usize>,
    author_map: HashMap<String, Vec<usize>>,
}

impl Posts {
    pub fn new(data: Vec<Post>, url_map: HashMap<String, usize>) -> Self {
        let author_map = HashMap::new();
        Self { data, url_map, author_map }
    }

    pub fn get(&self, path: &str) -> Option<&Post> {
        self.url_map.get(path).map(|id| &self.data[*id])
    }

    pub fn get_index(&self, count: usize, from_old_to_new: bool) -> Option<List> {
        if self.data.is_empty() {
            return None;
        }

        let data = if from_old_to_new {
            self.data.iter().take(count).collect()
        } else {
            self.data.iter().rev().take(count).collect()
        };

        Some(List::Index { data })
    }
}
