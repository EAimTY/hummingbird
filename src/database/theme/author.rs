use super::Theme;
use hyper::{Body, Response};

impl Theme {
    pub fn render_author(&self, author: &str) -> Response<Body> {
        Response::new(Body::empty())
    }
}
