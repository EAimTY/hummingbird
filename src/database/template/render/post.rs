use super::Template;
use crate::database::Post;
use hyper::{Body, Response};

impl Template {
    pub fn render_post(&self, post: &Post) -> Response<Body> {
        Response::new(Body::from(post.content.to_owned()))
    }
}
