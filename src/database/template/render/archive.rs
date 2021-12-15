use super::{
    data_map::{DocumentDataMap, SiteDataMap, SummaryDataMap},
    Template,
};
use crate::database::{Post, TimeRange};
use hyper::{Body, Response};

impl Template {
    pub fn render_archive(&self, time_range: TimeRange, posts: Vec<&Post>) -> Response<Body> {
        let time_range = match time_range {
            TimeRange::Year { year, .. } => year.to_string(),
            TimeRange::Month { year, month, .. } => format!("{}-{}", year, month),
            TimeRange::Free { .. } => unreachable!(),
        };

        let site_data = SiteDataMap::from_config();
        let document_data = DocumentDataMap::from_time_range(&time_range);

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
