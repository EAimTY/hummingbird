use super::Data;

pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Data) -> String {
        match data {
            Data::Post(post) => post.content.clone(),
            Data::Archive(_archive) => todo!(),
        }
    }
}
