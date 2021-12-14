use super::{Params, Template};
use crate::database::Page;
use hyper::{Body, Response};

impl Template {
    pub fn render_page(&self, page: &Page) -> Response<Body> {
        let params = Params::from_page(page);

        let header = self.header(&params);
        let footer = self.footer(&params);
        let page = self.page(&params);

        Response::new(Body::from(format!("{}{}{}", header, page, footer)))
    }
}
