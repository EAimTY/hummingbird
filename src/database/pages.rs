use super::data::Page;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pages {
    pub data: Vec<Page>,
    pub url_map: HashMap<String, usize>,
}

impl Pages {
    pub fn get(&self, path: &str) -> Option<&Page> {
        self.url_map.get(path).map(|id| &self.data[*id])
    }
}
