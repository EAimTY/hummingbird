use super::{Params, Template};
use crate::database::Post;
use hyper::{Body, Response};

impl Template {
    pub fn render_index(&self, index: Vec<&Post>) -> Response<Body> {
        let params_site = Params::from_site("index");

        let header = self.header(&params_site);
        let footer = self.footer(&params_site);

        let index = index
            .iter()
            .map(|post| {
                let params = Params::from_post_to_summary(post);
                self.summary(&params)
            })
            .collect::<String>();

        Response::new(Body::from(format!("{}{}{}", header, index, footer)))
    }
}
