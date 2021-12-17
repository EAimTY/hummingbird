use crate::DatabaseManager;
use hyper::{Body, Request, Response};
use std::ops::Deref;

pub async fn handle(req: &Request<Body>) -> Response<Body> {
    let db = DatabaseManager::read().await;

    db.template.render_not_found(db.deref(), req)
}
