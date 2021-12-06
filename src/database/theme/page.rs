use super::Theme;
use crate::database::Page;
use hyper::{Body, Response};

impl Theme {
    pub fn render_page(&self, page: &Page) -> Response<Body> {
        Response::new(Body::from(page.content.to_owned()))
    }
}
