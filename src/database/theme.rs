use super::Query;

pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Query) -> String {
        match data {
            Query::Post(post) => post.content.clone(),
            Query::Archive(_archive) => todo!(),
        }
    }
}
