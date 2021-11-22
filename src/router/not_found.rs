use crate::{Data, Database};
use hyper::{Body, Request, Response};

pub async fn handle(db: &Database, _req: &Request<Body>) -> Response<Body> {
    let db = db.data.read().await;
    db.theme.render(Data::NotFound)
}
