use super::{
    data_map::{DocumentDataMap, SiteDataMap, SummaryDataMap},
    Template,
};
use crate::database::{Database, ListInfo, Post};
use hyper::{Body, Request, Response};

impl Template {
    pub fn render_index(
        &self,
        db: &Database,
        req: &Request<Body>,
        posts: Vec<&Post>,
        list_info: ListInfo,
    ) -> Response<Body> {
        let site_data = SiteDataMap::from_config_and_db(db);
        let document_data = DocumentDataMap::from_index(req, list_info);

        let header = self.header(&site_data, &document_data);
        let posts = posts
            .iter()
            .map(|post| {
                let summary_data = SummaryDataMap::from_post(post);
                self.summary(&site_data, &document_data, &summary_data)
            })
            .collect::<String>();
        let page_nav = self.page_nav(&site_data, &document_data);
        let footer = self.footer(&site_data, &document_data);

        Response::new(Body::from(format!(
            "{}{}{}{}",
            header, posts, page_nav, footer
        )))
    }
}
