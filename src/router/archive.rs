use crate::{
    database::{ListInfo, TimeRange},
    router, DatabaseManager,
};
use hyper::{Body, Method, Request, Response};

pub async fn handle(
    req: &Request<Body>,
    year: &str,
    month: Option<&str>,
) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let time_range = TimeRange::from_year_month(year, month)?;

        let (current_page, page_num_pos_in_url, is_page_num_the_first_param_in_query) =
            router::get_page_num_and_pos_in_url(req.uri());

        let (posts, total_article_counts) = db.posts.get_time_range(&time_range, current_page)?;

        let list_info = ListInfo::new(
            current_page,
            total_article_counts,
            page_num_pos_in_url,
            is_page_num_the_first_param_in_query,
        );

        let res = db
            .template
            .render_archive(req, time_range, posts, list_info);

        return Some(res);
    }
    None
}
