use crate::db;
use axum::extract::Path;
use axum::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;

pub async fn init() {
    let mut db = db::Db::new("user", "token", "http://127.0.0.1:1080").await;
    db.fetch("")
        .await;
    db.get_posts().await;
    let posts = db.return_post();

    let app = route(
        "/:path",
        get(move |Path(path): Path<String>| async {
            let mut content = String::new();
            println!("{}", path);
            let path = PathBuf::from(path);
            if path.extension() == Some(OsStr::new("html")) {
                let title = path.file_stem().unwrap().to_str().unwrap();
                println!("{}", title);
                for post in posts {
                    println!("{}", post.title);
                    if title == post.title {
                        content = post.content;
                    }
                }
            }
            content
        }),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
