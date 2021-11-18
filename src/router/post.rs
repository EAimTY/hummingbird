use crate::database::Database;
use hyper::{Body, Request, Response};

pub async fn handle(database: &Database, request: &Request<Body>) -> Option<Response<Body>> {
    let path = request.uri().path();
    database.get_post(path).await
}
