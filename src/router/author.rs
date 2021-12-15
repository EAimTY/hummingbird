use crate::{router, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, author: &str) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let post_ids = db.authors.get_posts(author)?;

        let page_num = req
            .uri()
            .query()
            .map_or(1, |query| router::get_page_num(query).unwrap_or(1));

        let posts = db.posts.get_multi(post_ids, page_num)?;

        let res = db.template.render_author(author, posts);
        return Some(res);
    }
    None
}
