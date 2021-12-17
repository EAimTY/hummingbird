use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};
use std::ops::Deref;

pub async fn handle(req: &Request<Body>, page_id: usize) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let page = db.pages.get(page_id);

        let res = db.template.render_page(db.deref(), req, page);
        return Some(res);
    }
    None
}
