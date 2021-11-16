use crate::database::Database;
use hyper::{Body, Request, Response};

pub async fn get(mut database: Database, _request: Request<Body>) -> Response<Body> {
    database.update().await.unwrap();
    Response::new(Body::from("done"))
}
