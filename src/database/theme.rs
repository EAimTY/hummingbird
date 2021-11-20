use super::{data::List, Data};
use hyper::{Body, Response};

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
            Data::List(list) => match list {
                List::Index { data } => Response::new(Body::from(
                    data.into_iter()
                        .map(|post| format!("{}\n{}\n\n", post.title.clone(), post.content.clone()))
                        .collect::<String>(),
                )),
                _ => todo!(),
            },
        }
    }
}
