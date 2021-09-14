use crate::{config, op::Op};
use anyhow::Result;
use axum::{extract::Path, handler::get, Router as AxumRouter};
use tokio::sync::{mpsc, oneshot};

pub struct Router;

impl Router {
    pub async fn run(config: &config::Settings, op_sender: mpsc::Sender<Op>) -> Result<()> {
        tokio::spawn(async move {
            let op_sender_get_post = op_sender.clone();
            let op_sender_get_page = op_sender.clone();

            let app = AxumRouter::new()
                .route(
                    "/post/:path",
                    get(|Path(path): Path<String>| async move {
                        let op_channel = oneshot::channel();
                        Router::get_post(path, op_channel, op_sender_get_post).await
                    }),
                )
                .route(
                    "/page/:path",
                    get(|Path(path): Path<String>| async move {
                        let op_channel = oneshot::channel();
                        Router::get_post(path, op_channel, op_sender_get_page).await
                    }),
                );

            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
        Ok(())
    }

    async fn get_post(
        path: String,
        channel: (oneshot::Sender<String>, oneshot::Receiver<String>),
        op_sender: mpsc::Sender<Op>,
    ) -> String {
        if path.ends_with(".html") {
            op_sender
                .send(Op::GetPost {
                    title: path[..path.len() - 5].to_string(),
                    channel_sender: channel.0,
                })
                .await;
        }
        channel.1.await.unwrap_or(String::from("failed"))
    }

    async fn get_page(
        path: String,
        channel: (oneshot::Sender<String>, oneshot::Receiver<String>),
        op_sender: mpsc::Sender<Op>,
    ) -> String {
        if path.ends_with(".html") {
            op_sender
                .send(Op::GetPage {
                    title: path[..path.len() - 5].to_string(),
                    channel_sender: channel.0,
                })
                .await;
        }
        channel.1.await.unwrap_or(String::from("failed"))
    }
}
