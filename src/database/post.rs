use super::Database;

#[derive(Clone, Debug)]
pub struct Post {
    pub content: String,
}

impl Database {
    pub fn get_post(&self, title: String) -> Post {
        self.posts.get(&title).unwrap().clone()
    }
}
