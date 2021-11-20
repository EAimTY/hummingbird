use crate::data::Page;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pages {
    data: Vec<Page>,
    url_map: HashMap<String, usize>,
}

impl Pages {
    pub fn new(data: Vec<Page>, url_map: HashMap<String, usize>) -> Self {
        Self { data, url_map }
    }

    pub fn get(&self, path: &str) -> Option<&Page> {
        self.url_map.get(path).map(|id| &self.data[*id])
    }
}
