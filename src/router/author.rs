use crate::Database;
use hyper::{Body, Method, Request, Response};

pub async fn handle(db: &Database, req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = db.data.read().await;

        let path = req.uri().path();

        let res = db
            .posts
            .get_author(path)
            .map(|author| db.theme.render(author));

        return res;
    }
    None
}
