use super::{
    data_map::{DocumentDataMap, PageDataMap, SiteDataMap},
    Template,
};
use crate::database::Page;
use hyper::{Body, Request, Response};

impl Template {
    pub fn render_page(&self, req: &Request<Body>, page: &Page) -> Response<Body> {
        let site_data = SiteDataMap::from_config();
        let document_data = DocumentDataMap::from_page(req, page);

        let page_data = PageDataMap::from_page(page);

        let header = self.header(&site_data, &document_data);
        let page = self.page(&site_data, &document_data, &page_data);
        let footer = self.footer(&site_data, &document_data);

        Response::new(Body::from(format!("{}{}{}", header, page, footer)))
    }
}
