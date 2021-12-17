use crate::{
    database::{ListInfo, PostFilter},
    router, DatabaseManager,
};
use hyper::{Body, Method, Request, Response};
use std::ops::Deref;

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let query = req.uri().query()?;
        let filters = PostFilter::from_uri_query(query)?;

        let (current_page_num_in_list, page_num_pos_in_url, is_page_num_the_first_param_in_query) =
            router::get_page_num_and_pos_in_url(req.uri());

        let (posts, total_num_of_articles_in_list) =
            db.posts.search(&filters, current_page_num_in_list)?;

        let list_info = ListInfo::new(
            current_page_num_in_list,
            total_num_of_articles_in_list,
            page_num_pos_in_url,
            is_page_num_the_first_param_in_query,
        );

        let res = db
            .template
            .render_search(db.deref(), req, filters, posts, list_info);

        return Some(res);
    }
    None
}
