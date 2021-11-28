use crate::{database::DataType, DatabaseManager};
use hyper::{Body, Request, Response};

pub async fn handle(_req: &Request<Body>) -> Response<Body> {
    let db = DatabaseManager::read().await;
    db.theme.render(DataType::NotFound)
}
