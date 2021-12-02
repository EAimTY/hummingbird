use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, post_id: usize) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let res = db.posts.get(post_id).map(|post| db.theme.render(post));
        return res;
    }
    None
}
