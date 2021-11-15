use crate::database::Database;
use hyper::{Body, Request, Response};

pub async fn get(database: Database, request: Request<Body>) -> Response<Body> {
    let uri = request.uri().to_string();
    let post = database.get_post(&uri).await;

    Response::new(Body::from(post))
}
