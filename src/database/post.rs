use super::Database;

#[derive(Debug)]
pub struct Post {
    pub content: String,
}

impl Database {
    pub fn get_post(&self, title: String) -> String {
        if let Some(post) = self.posts.get(&title) {
            post.content.clone()
        } else {
            String::from("not found in database")
        }
    }
}
