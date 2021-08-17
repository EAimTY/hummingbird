use axum::prelude::*;
use axum::extract::Path;

pub async fn create_router() {
    let app = route("/", get(root))
        .route("/post/:name", get(post));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post(Path(name): Path<String>) -> String {
    name
}

async fn root() -> &'static str {
    "Hello, World!"
}
