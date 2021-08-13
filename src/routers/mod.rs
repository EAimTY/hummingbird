use axum::prelude::*;

mod post;
mod page;
mod root;

pub async fn router() {
    let app =
        route("/", get(root::root))
        .route("/post/:name", get(post::post))
        .route("/page/:name", get(page::page));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
