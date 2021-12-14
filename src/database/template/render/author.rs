use super::{Params, Template};
use crate::database::Post;
use hyper::{Body, Response};

impl Template {
    pub fn render_author(&self, author: &str, posts: Vec<&Post>) -> Response<Body> {
        let params_site = Params::from_site(author);

        let header = self.header(&params_site);
        let footer = self.footer(&params_site);

        let posts = posts
            .iter()
            .map(|post| {
                let params = Params::from_post_to_summary(post);
                self.summary(&params)
            })
            .collect::<String>();

        Response::new(Body::from(format!("{}{}{}", header, posts, footer)))
    }
}
