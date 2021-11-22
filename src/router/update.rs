use crate::{data::UpdateResult, Config, Data, Database};
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
                let db = db.data.read().await;

                let res = db
                    .theme
                    .render(Data::Update(UpdateResult::PermissionDenied));

                return Some(res);
            }
        }
    } else if req.method() == Method::GET {
        update = true;
    }

    if update {
        let mut db = db.data.write().await;

        let result = db
            .update()
            .await
            .map_or_else(UpdateResult::Error, |_| UpdateResult::Success);
        let res = db.theme.render(Data::Update(result));
        return Some(res);
    }

    None
}
