use super::{
    data_map::{DocumentDataMap, SiteDataMap, SummaryDataMap},
    Template,
};
use crate::database::Post;
use hyper::{Body, Response};

impl Template {
    pub fn render_index(&self, posts: Vec<&Post>) -> Response<Body> {
        let site_data = SiteDataMap::from_config();
        let document_data = DocumentDataMap::from_index();

        let header = self.header(&site_data, &document_data);
        let posts = posts
            .iter()
            .map(|post| {
                let summary_data = SummaryDataMap::from_post(post);
                self.summary(&site_data, &document_data, &summary_data)
            })
            .collect::<String>();
        let footer = self.footer(&site_data, &document_data);

        Response::new(Body::from(format!("{}{}{}", header, posts, footer)))
    }
}
