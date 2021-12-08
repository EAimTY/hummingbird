use super::Theme;
use crate::database::Post;
use hyper::{Body, Response};

impl Theme {
    pub fn render_search(&self, result: Vec<&Post>) -> Response<Body> {
        Response::new(Body::from(
            result
                .into_iter()
                .map(|post| format!("{}\n{}\n\n", post.title.to_owned(), post.content.to_owned()))
                .collect::<String>(),
        ))
    }
}
