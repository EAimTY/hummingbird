use crate::{database::TimeRange, router, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(
    req: &Request<Body>,
    year: &str,
    month: Option<&str>,
) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let time_range = TimeRange::from_year_month(year, month)?;

        let page_num = req
            .uri()
            .query()
            .map_or(1, |query| router::get_page_num(query).unwrap_or(1));

        let posts = db.posts.get_time_range(&time_range, page_num)?;

        let res = db.template.render_archive(time_range, posts);
        return Some(res);
    }
    None
}
