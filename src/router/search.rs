use crate::{database::PostFilter, router, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let query = req.uri().query()?;
        let filters = PostFilter::from_uri_query(query)?;

        let page_num = req
            .uri()
            .query()
            .map_or(1, |query| router::get_page_num(query).unwrap_or(1));

        let search_result = db.posts.search(&filters, page_num)?;

        let res = db.template.render_search(filters, search_result);
        return Some(res);
    }
    None
}
