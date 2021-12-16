use crate::{database::PostFilter, router, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let query = req.uri().query()?;
        let filters = PostFilter::from_uri_query(query)?;

        let current_page = req
            .uri()
            .query()
            .map_or(1, |query| router::get_current_page(query).unwrap_or(1));

        let (posts, total_page) = db.posts.search(&filters, current_page)?;

        let res = db
            .template
            .render_search(filters, posts, current_page, total_page);
        return Some(res);
    }
    None
}
