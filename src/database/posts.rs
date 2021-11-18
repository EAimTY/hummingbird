use super::data::Post;
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
}
