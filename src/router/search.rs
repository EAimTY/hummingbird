use crate::{database::PostFilter, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let query = req.uri().query()?;
        let filters = PostFilter::from_uri_query(query)?;
        let search_result = db.posts.filter(&filters)?;

        let res = db.template.render_search(filters, search_result);
        return Some(res);
    }
    None
}
