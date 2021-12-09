use crate::{database::TimeRange, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(
    req: &Request<Body>,
    year: &str,
    month: Option<&str>,
) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        if let Some(time_range) = TimeRange::from_year_month(year, month) {
            if let Some(posts) = db.posts.get_time_range(&time_range) {
                let res = db.theme.render_archive(time_range, posts);
                return Some(res);
            }
        }
    }
    None
}
