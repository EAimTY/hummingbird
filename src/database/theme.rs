use super::Data;

#[derive(Debug)]
pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Data) -> String {
        match data {
            Data::Post(post) => post.content.clone(),
            Data::Page(page) => page.content.clone(),
            Data::Archive(_archive) => todo!(),
        }
    }
}
