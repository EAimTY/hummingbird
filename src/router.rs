use crate::{config, db};
use anyhow::Result;
use axum::{extract::Path, handler::get, Router as AxumRouter};
use tokio::sync::mpsc;

pub struct Router;

impl Router {
    pub async fn run(config: &config::Settings, tx: mpsc::Sender<db::DbOp>) -> Result<()> {
        tokio::spawn(async move {
            let tx_get_post = tx.clone();
            let tx_get_page = tx.clone();
            
            let app = AxumRouter::new()
                .route(
                    "/post/:path",
                    get(|Path(path): Path<String>| async move {
                        Router::get_post(path, tx_get_post).await
                    }),
                )
                .route(
                    "/page/:title",
                    get(|Path(title): Path<String>| async move {
                        Router::get_page(title, tx_get_page).await
                    }),
                );

            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
        Ok(())
    }

    async fn get_post(path: String, tx: mpsc::Sender<db::DbOp>) -> String {
        if path.ends_with(".html") {
            tx.send(db::DbOp::GetPost(path[..path.len() - 5].to_string())).await;
        }
        String::from("post")
    }

    async fn get_page(title: String, tx: mpsc::Sender<db::DbOp>) -> String {
        String::from("page")
    }
}
