use crate::Database;
use hyper::{Body, Method, Request, Response};

pub async fn handle(db: &Database, req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = db.data.read().await;

        let res = db.posts.get_index().map(|index| db.theme.render(index));
        return res;
    }
    None
}
