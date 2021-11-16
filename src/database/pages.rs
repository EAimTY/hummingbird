use super::data::Page;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pages {
    pub data: Vec<Page>,
    pub url_map: HashMap<String, usize>,
}

impl Pages {
    pub fn get_page(&self, path: &str) -> &Page {
        let id = self.url_map.get(path).unwrap();
        &self.data[*id]
    }
}
