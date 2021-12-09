use super::Theme;
use crate::database::{Post, PostFilter};
use hyper::{Body, Response};

impl Theme {
    pub fn render_search(&self, _filters: Vec<PostFilter>, result: Vec<&Post>) -> Response<Body> {
        Response::new(Body::from(
            result
                .into_iter()
                .map(|post| format!("{}\n{}\n\n", post.title.to_owned(), post.content.to_owned()))
                .collect::<String>(),
        ))
    }
}
