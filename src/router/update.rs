use crate::database::Database;
use hyper::{Body, Request, Response};

pub async fn handle(database: &mut Database, _request: &Request<Body>) -> Option<Response<Body>> {
    if let Ok(_) = database.update().await {
        Some(Response::new(Body::from("update done")))
    } else {
        Some(Response::new(Body::from("update failed")))
    }
}
