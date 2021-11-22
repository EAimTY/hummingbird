use crate::data::Page;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pages {
    data: Vec<Page>,
    url_map: HashMap<String, usize>,
    author_map: HashMap<String, Vec<usize>>,
}

impl Pages {
    pub fn new(data: Vec<Page>, url_map: HashMap<String, usize>) -> Self {
        let author_map = HashMap::new();
        Self { data, url_map, author_map }
    }

    pub fn get(&self, path: &str) -> Option<&Page> {
        self.url_map.get(path).map(|id| &self.data[*id])
    }
}
