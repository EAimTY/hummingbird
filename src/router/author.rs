use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, author: &str) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let res = db.theme.render_author(author);
        return Some(res);
    }
    None
}
