use super::{
    data_map::{DocumentDataMap, PostDataMap, SiteDataMap},
    Template,
};
use crate::database::Post;
use hyper::{Body, Request, Response};

impl Template {
    pub fn render_post(&self, req: &Request<Body>, post: &Post) -> Response<Body> {
        let site_data = SiteDataMap::from_config_and_db();
        let document_data = DocumentDataMap::from_post(req, post);

        let post_data = PostDataMap::from_post(post);

        let header = self.header(&site_data, &document_data);
        let post = self.post(&site_data, &document_data, &post_data);
        let footer = self.footer(&site_data, &document_data);

        Response::new(Body::from(format!("{}{}{}", header, post, footer)))
    }
}
