use crate::Database;
use hyper::{Body, Request, Response};

pub async fn handle(_db: &Database, _req: &Request<Body>) -> Option<Response<Body>> {
    todo!()
}
