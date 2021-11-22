use crate::{data::UpdateResult, Data};
use hyper::{Body, Response};

#[derive(Debug)]
pub struct Theme {}

impl Theme {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Data) -> Response<Body> {
        match data {
            Data::Post(post) => Response::new(Body::from(post.content().to_owned())),
            Data::Page(page) => Response::new(Body::from(page.content().to_owned())),
            Data::Update(result) => match result {
                UpdateResult::Success => Response::new(Body::from("ok")),
                UpdateResult::PermissionDenied => Response::builder()
                    .status(403)
                    .body(Body::from("permission denied"))
                    .unwrap(),
                UpdateResult::Error(error) => Response::new(Body::from(error.to_string())),
            },
            Data::Index { data } => Response::new(Body::from(
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
            Data::Author { data, author } => {
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
            Data::Time { data, time } => todo!(),
            Data::NotFound => Response::builder()
                .status(404)
                .body(Body::from("not found"))
                .unwrap(),
        }
    }
}
