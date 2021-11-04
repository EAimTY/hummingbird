use super::{
    query::{PostData, Query},
    Database,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Posts {
    data: Vec<Post>,
    map: HashMap<String, usize>,
}

impl Posts {
    pub fn new() -> Self {
        Posts {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub create_time: i64,
    pub modify_time: i64,
}

impl Database {
    pub fn get_post(&self, path: &str) -> String {
        let id = self.posts.map.get(path).unwrap();
        let post_data = &self.posts.data[*id];
        self.theme.render(Query::Post(PostData { data: post_data }))
    }
}
