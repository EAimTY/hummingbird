use super::{Params, Template};
use crate::database::{Post, TimeRange};
use hyper::{Body, Response};

impl Template {
    pub fn render_archive(&self, time_range: TimeRange, posts: Vec<&Post>) -> Response<Body> {
        let time_range = match time_range {
            TimeRange::Year { year, .. } => year.to_string(),
            TimeRange::Month { year, month, .. } => format!("{}-{}", year, month),
            TimeRange::Free { .. } => unreachable!(),
        };

        let params_site = Params::from_site(&time_range);

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
