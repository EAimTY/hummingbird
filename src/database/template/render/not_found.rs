use super::{
    data_map::{DocumentDataMap, SiteDataMap},
    Template,
};
use hyper::{Body, Response};

impl Template {
    pub fn render_not_found(&self) -> Response<Body> {
        let site_data = SiteDataMap::from_config();
        let document_data = DocumentDataMap::from_not_found();

        let header = self.header(&site_data, &document_data);
        let not_found = self.not_found(&site_data, &document_data);
        let footer = self.footer(&site_data, &document_data);

        Response::builder()
            .status(404)
            .body(Body::from(format!("{}{}{}", header, not_found, footer)))
            .unwrap()
    }
}
