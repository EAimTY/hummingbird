use super::Theme;
use crate::database::Post;
use hyper::{Body, Response};

impl Theme {
    pub fn render_post(&self, post: &Post) -> Response<Body> {
        Response::new(Body::from(post.content.to_owned()))
    }
}
