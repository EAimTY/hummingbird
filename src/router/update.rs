use crate::{
    database::{data_type::UpdateResult, DataType},
    Config, Database,
};
use hyper::{
    body::{self, Buf},
    Body, Method, Request, Response,
};

pub async fn handle(req: &mut Request<Body>) -> Option<Response<Body>> {
    let mut update = false;

    if let Some(update_token) = &Config::read().settings.update_token {
        if req.method() == Method::POST {
            if let Ok(body) = body::aggregate(req.body_mut()).await {
                if update_token.as_bytes() == body.chunk() {
                    update = true;
                }
            }

            if !update {
                let db = Database::read().await;

                let res = db
                    .theme
                    .render(DataType::Update(UpdateResult::PermissionDenied));

                return Some(res);
            }
        }
    } else if req.method() == Method::GET {
        update = true;
    }

    if update {
        let mut db = Database::write().await;

        let result = db
            .update()
            .await
            .map_or_else(UpdateResult::Error, |_| UpdateResult::Success);
        let res = db.theme.render(DataType::Update(result));
        return Some(res);
    }

    None
}
