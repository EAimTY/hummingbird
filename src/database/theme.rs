use crate::{data::List, Data};
use hyper::{Body, Response};

#[derive(Debug)]
pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Data) -> Response<Body> {
        match data {
            Data::Post(post) => Response::new(Body::from(post.content().to_owned())),
            Data::Page(page) => Response::new(Body::from(page.content().to_owned())),
            Data::List(list) => match list {
                List::Index { data } => Response::new(Body::from(
                    data.into_iter()
                        .map(|post| {
                            format!(
                                "{}\n{}\n\n",
                                post.title().to_owned(),
                                post.content().to_owned()
                            )
                        })
                        .collect::<String>(),
                )),
                List::Author { data, author } => {
                    let list = data
                        .into_iter()
                        .map(|post| {
                            format!(
                                "{}\n{}\n\n",
                                post.title().to_owned(),
                                post.content().to_owned()
                            )
                        })
                        .collect::<String>();
                    Response::new(Body::from(format!("{}\n\n{}", author, list)))
                }
                _ => todo!(),
            },
        }
    }
}
