use crate::{Config, Database};
use hyper::{
    body::{self, Buf},
    Body, Method, Request, Response,
};

pub async fn handle(db: &mut Database, req: &mut Request<Body>) -> Option<Response<Body>> {
    let mut update = false;

    if let Some(update_token) = &Config::read().settings.update_token {
        if req.method() == Method::POST {
            if let Ok(body) = body::aggregate(req.body_mut()).await {
                if update_token.as_bytes() == body.chunk() {
                    update = true;
                }
            }

            if !update {
                return Some(Response::builder().status(403).body(Body::empty()).unwrap());
            }
        }
    } else if req.method() == Method::GET {
        update = true;
    }

    if update {
        if db.update().await.is_ok() {
            return Some(Response::new(Body::from("update done")));
        } else {
            return Some(Response::new(Body::from("update failed")));
        }
    }

    None
}
