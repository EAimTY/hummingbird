use crate::db;
use axum::extract::Path;
use axum::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;

pub async fn init() {
    let mut db = db::Db::new(
        "",
        "",
        "",
        "",
    )
    .await;
    db.fetch().await;
    let pages = db.get_pages();
    let posts = db.get_posts();

    let app = route(
        "/:path",
        get(move |Path(path): Path<String>| async { serve_page(path, pages).await }),
    )
    .route(
        "/posts/:path",
        get(move |Path(path): Path<String>| async { serve_post(path, posts).await }),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_page(path: String, pages: Vec<db::Page>) -> String {
    let path = PathBuf::from(path);
    let mut content: Option<String> = None;
    if path.extension() == Some(OsStr::new("html")) {
        let title = path.file_stem().unwrap().to_str().unwrap();
        for page in pages {
            if title == page.title {
                content = Some(page.content.clone());
                break;
            }
        }
    }
    if let Some(content) = content {
        content
    } else {
        String::from("Page Not Found")
    }
}

async fn serve_post(path: String, posts: Vec<db::Post>) -> String {
    let path = PathBuf::from(path);
    let mut content: Option<String> = None;
    if path.extension() == Some(OsStr::new("html")) {
        let title = path.file_stem().unwrap().to_str().unwrap();
        for post in posts {
            if title == post.title {
                content = Some(post.content.clone());
                break;
            }
        }
    }
    if let Some(content) = content {
        content
    } else {
        String::from("Post Not Found")
    }
}
