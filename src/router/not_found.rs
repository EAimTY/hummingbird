use crate::Database;
use hyper::{Body, Request, Response};

pub async fn handle(db: &Database, _req: &Request<Body>) -> Response<Body> {
    db.not_found().await
}
