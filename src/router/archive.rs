use crate::{
    database::{ListInfo, TimeRange},
    router, DatabaseManager,
};
use hyper::{Body, Method, Request, Response};
use std::ops::Deref;

pub async fn handle(
    req: &Request<Body>,
    year: &str,
    month: Option<&str>,
) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let time_range = TimeRange::from_year_month(year, month)?;

        let (current_page_num_in_list, page_num_pos_in_url, is_page_num_the_first_param_in_query) =
            router::get_page_num_and_pos_in_url(req.uri());

        let (posts, total_num_of_articles_in_list) = db
            .posts
            .get_time_range(&time_range, current_page_num_in_list)?;

        let list_info = ListInfo::new(
            current_page_num_in_list,
            total_num_of_articles_in_list,
            page_num_pos_in_url,
            is_page_num_the_first_param_in_query,
        );

        let res = db
            .template
            .render_archive(db.deref(), req, time_range, posts, list_info);

        return Some(res);
    }
    None
}
