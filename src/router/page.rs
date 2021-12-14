use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, page_id: usize) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let page = db.pages.get(page_id);

        let res = db.template.render_page(page);
        return Some(res);
    }
    None
}
