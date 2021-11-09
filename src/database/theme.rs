use super::Query;

pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, _data: Query) -> String {
        String::from("rendered")
    }
}
