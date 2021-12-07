use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(
    req: &Request<Body>,
    year: &str,
    month: Option<&str>,
) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let res = db.theme.render_archive();
        return Some(res);
    }
    None
}
