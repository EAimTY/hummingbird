use crate::Database;
use hyper::{Body, Method, Request, Response};

pub async fn handle(db: &Database, req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        return db.get_index().await;
    }
    None
}
