use crate::{database::PostFilter, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let query = req.uri().query()?;
        let filters = PostFilter::from_uri_query(query)?;

        if let Some(result) = db.posts.filter(&filters) {
            let res = db.theme.render_search(filters, result);
            return Some(res);
        }
    }
    None
}
