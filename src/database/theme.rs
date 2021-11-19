use hyper::{Body, Response};

use super::Data;

#[derive(Debug)]
pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Data) -> Response<Body> {
        match data {
            Data::Post(post) => Response::new(Body::from(post.content.clone())),
            Data::Page(page) => Response::new(Body::from(page.content.clone())),
            Data::List(_list) => todo!(),
        }
    }
}
