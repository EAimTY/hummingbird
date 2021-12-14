use super::Template;
use crate::database::{Post, TimeRange};
use hyper::{Body, Response};

impl Template {
    pub fn render_archive(&self, time_range: TimeRange, posts: Vec<&Post>) -> Response<Body> {
        let time_range = match time_range {
            TimeRange::Year { year, .. } => year.to_string(),
            TimeRange::Month { year, month, .. } => format!("{}-{}", year, month),
            TimeRange::Free { .. } => unreachable!(),
        };

        Response::new(Body::from(format!(
            "{}\n\n{}",
            time_range,
            posts
                .into_iter()
                .map(|post| format!("{}\n{}\n\n", post.title.to_owned(), post.content.to_owned()))
                .collect::<String>()
        )))
    }
}
