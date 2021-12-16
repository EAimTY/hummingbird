use crate::DatabaseManager;
use hyper::{Body, Request, Response};

pub async fn handle(req: &Request<Body>) -> Response<Body> {
    let db = DatabaseManager::read().await;

    db.template.render_not_found(req)
}
