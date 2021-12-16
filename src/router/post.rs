use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, post_id: usize) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let post = db.posts.get(post_id);

        let res = db.template.render_post(req, post);
        return Some(res);
    }
    None
}
