use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let index = db.posts.get_index();

        let res = db.theme.render_index(index);
        return Some(res);
    }
    None
}
