use super::Theme;
use hyper::{Body, Response};

impl Theme {
    pub fn render_archive(&self) -> Response<Body> {
        Response::new(Body::empty())
    }
}
