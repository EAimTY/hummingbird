use super::data::Post;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Posts {
    pub data: Vec<Post>,
    pub url_map: HashMap<String, usize>,
}

impl Posts {
    pub fn get_post(&self, path: &str) -> &Post {
        let id = self.url_map.get(path).unwrap();
        &self.data[*id]
    }
}
