use crate::DatabaseManager;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, author: &str) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let post_ids = db.authors.get_posts(author);
        let posts = db.posts.get_multi(post_ids);

        let res = db.theme.render_author(author, posts);
        return Some(res);
    }
    None
}
