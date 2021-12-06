use super::Theme;
use crate::database::Post;
use hyper::{Body, Response};

impl Theme {
    pub fn render_index(&self, index: Vec<&Post>) -> Response<Body> {
        Response::new(Body::from(
            index
                .into_iter()
                .map(|post| format!("{}\n{}\n\n", post.title.to_owned(), post.content.to_owned()))
                .collect::<String>(),
        ))
    }
}
