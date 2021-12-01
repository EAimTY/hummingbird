use super::{data_type::UpdateResult, DataType};
use hyper::{Body, Response};

#[derive(Clone, Debug)]
pub struct Theme {}

impl Theme {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: DataType) -> Response<Body> {
        match data {
            DataType::Post(post) => Response::new(Body::from(post.content.to_owned())),
            DataType::Page(page) => Response::new(Body::from(page.content.to_owned())),
            DataType::Update(result) => match result {
                UpdateResult::Success => Response::new(Body::from("ok")),
                UpdateResult::PermissionDenied => Response::builder()
                    .status(403)
                    .body(Body::from("permission denied"))
                    .unwrap(),
                UpdateResult::Error(error) => Response::new(Body::from(error.to_string())),
            },
            DataType::Index { data } => Response::new(Body::from(
                data.into_iter()
                    .map(|post| {
                        format!("{}\n{}\n\n", post.title.to_owned(), post.content.to_owned())
                    })
                    .collect::<String>(),
            )),
            DataType::NotFound => Response::builder()
                .status(404)
                .body(Body::from("not found"))
                .unwrap(),
        }
    }
}
