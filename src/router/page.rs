use crate::database::Database;
use hyper::{Body, Method, Request, Response};

pub async fn handle(database: &Database, request: &Request<Body>) -> Option<Response<Body>> {
    if request.method() == Method::GET {
        let path = request.uri().path();
        return database.get_page(path).await;
    }
    None
}
