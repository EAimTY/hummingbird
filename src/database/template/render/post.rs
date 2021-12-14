use super::{Params, Template};
use crate::database::Post;
use hyper::{Body, Response};

impl Template {
    pub fn render_post(&self, post: &Post) -> Response<Body> {
        let params = Params::from_post(post);

        let header = self.header(&params);
        let footer = self.footer(&params);
        let post = self.post(&params);

        Response::new(Body::from(format!("{}{}{}", header, post, footer)))
    }
}
