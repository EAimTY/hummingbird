use crate::{database::DataType, Database};
use hyper::{Body, Request, Response};

pub async fn handle(_req: &Request<Body>) -> Response<Body> {
    let db = Database::read().await;
    db.theme.render(DataType::NotFound)
}
