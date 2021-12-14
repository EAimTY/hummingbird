use super::{Params, Template};
use crate::database::{Post, PostFilter};
use hyper::{Body, Response};

impl Template {
    pub fn render_search(&self, _filters: Vec<PostFilter>, result: Vec<&Post>) -> Response<Body> {
        let params_site = Params::from_site("search");

        let header = self.header(&params_site);
        let footer = self.footer(&params_site);

        let result = result
            .iter()
            .map(|post| {
                let params = Params::from_post_to_summary(post);
                self.summary(&params)
            })
            .collect::<String>();

        Response::new(Body::from(format!("{}{}{}", header, result, footer)))
    }
}
