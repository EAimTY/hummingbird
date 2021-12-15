use crate::{router, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let page_num = req
            .uri()
            .query()
            .map_or(1, |query| router::get_page_num(query).unwrap_or(1));

        let index = db.posts.get_index(page_num)?;

        let res = db.template.render_index(index);
        return Some(res);
    }
    None
}
