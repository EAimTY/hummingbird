use super::Theme;
use hyper::{Body, Response};

impl Theme {
    pub fn render_not_found(&self) -> Response<Body> {
        Response::builder()
            .status(404)
            .body(Body::from("not found"))
            .unwrap()
    }
}
