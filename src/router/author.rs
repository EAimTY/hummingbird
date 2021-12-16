use crate::{database::ListInfo, router, DatabaseManager};
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>, author: &str) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = DatabaseManager::read().await;

        let post_ids = db.authors.get_posts(author)?;

        let (current_page, page_num_pos_in_url, is_page_num_the_first_param_in_query) =
            router::get_page_num_and_pos_in_url(req.uri());

        let (posts, total_article_counts) = db.posts.get_multi(post_ids, current_page)?;

        let list_info = ListInfo::new(
            current_page,
            total_article_counts,
            page_num_pos_in_url,
            is_page_num_the_first_param_in_query,
        );

        let res = db.template.render_author(req, author, posts, list_info);

        return Some(res);
    }
    None
}
