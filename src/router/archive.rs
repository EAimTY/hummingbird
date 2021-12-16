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

        let current_page = req
            .uri()
            .query()
            .map_or(1, |query| router::get_current_page(query).unwrap_or(1));

        let (posts, total_page) = db.posts.get_time_range(&time_range, current_page)?;

        let res = db
            .template
            .render_archive(req, time_range, posts, current_page, total_page);
        return Some(res);
    }
    None
}
