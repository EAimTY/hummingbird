use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        if let Some(result) = db.posts.search(req.uri().query()?) {
            let res = db.theme.render_search(result);
            return Some(res);
        }
    }
    None
}
