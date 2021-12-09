use crate::{database::DatabaseUpdateResult, Config, DatabaseManager};
use hyper::{
    body::{self, Buf},
    Body, Method, Request, Response,
};

pub async fn handle(req: &mut Request<Body>) -> Option<Response<Body>> {
    let mut update = false;

    if let Some(update_token) = &Config::read().application.update_token {
        if req.method() == Method::POST {
            if let Ok(body) = body::aggregate(req.body_mut()).await {
                if update_token.as_bytes() == body.chunk() {
                    update = true;
                }
            }

            if !update {
                let db = DatabaseManager::read().await;

                let res = db
                    .theme
                    .render_update(DatabaseUpdateResult::PermissionDenied);

                return Some(res);
            }
        }
    } else if req.method() == Method::GET {
        update = true;
    }

    if update {
        let mut db = DatabaseManager::write().await;

        let result = db
            .update()
            .await
            .map_or_else(DatabaseUpdateResult::Error, |_| {
                DatabaseUpdateResult::Success
            });
        let res = db.theme.render_update(result);
        return Some(res);
    }

    None
}
