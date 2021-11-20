use super::data::{List, Post};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Posts {
    pub data: Vec<Post>,
    pub url_map: HashMap<String, usize>,
}

impl Posts {
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
