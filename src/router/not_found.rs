use crate::{database::DataType, Database};
use hyper::{Body, Request, Response};

pub async fn handle(db: &Database, _req: &Request<Body>) -> Response<Body> {
    let db = db.data.read().await;
    db.theme.render(DataType::NotFound)
}
