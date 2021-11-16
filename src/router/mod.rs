use crate::database::Database;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

mod page;
mod post;
mod update;

pub async fn handle(
    database: Database,
    request: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    Ok(if true {
        post::get(database, request).await
    } else {
        update::get(database, request).await;
        Response::new(Body::from("not found"))
    })
}
