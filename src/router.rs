use crate::{config, db};
use anyhow::Result;
use axum::{handler::get, Router};
use http::Uri;
use std::ffi::OsStr;
use std::path::PathBuf;

enum Requested {
    Other,
    Page,
    Post,
}

pub async fn init(config: config::Config) -> Result<()> {
    let mut db = db::Db::new(&config).await?;
    db.fetch().await?;
    let pages = db.get_pages();
    let posts = db.get_posts();

    let app = Router::new().nest(
        "/",
        get(|uri: Uri| async move {
            let uri = PathBuf::from(uri.path());
            let uri_parent = uri.parent();
            let requested = if let Some(uri_parent) = uri_parent {
                match uri_parent.as_os_str().to_str() {
                    Some("/page") => {
                        if uri.extension() == Some(OsStr::new("html")) {
                            Requested::Page
                        } else {
                            Requested::Other
                        }
                    }
                    Some("/post") => {
                        if uri.extension() == Some(OsStr::new("html")) {
                            Requested::Post
                        } else {
                            Requested::Other
                        }
                    }
                    _ => Requested::Other,
                }
            } else {
                Requested::Other
            };
            match requested {
                Requested::Page => {
                    let title = uri
                        .file_stem()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or("");
                    serve_page(title, pages).await
                }
                Requested::Post => {
                    let title = uri
                        .file_stem()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or("");
                    serve_post(title, posts).await
                }
                Requested::Other => String::from("Not Found"),
            }
        }),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn serve_page(title: &str, pages: Vec<db::Page>) -> String {
    let mut content: Option<String> = None;
    for page in pages {
        if title == page.title {
            content = Some(page.content.clone());
            break;
        }
    }
    if let Some(content) = content {
        content
    } else {
        String::from("Page Not Found")
    }
}

async fn serve_post(title: &str, posts: Vec<db::Post>) -> String {
    let mut content: Option<String> = None;
    for post in posts {
        if title == post.title {
            content = Some(post.content.clone());
            break;
        }
    }
    if let Some(content) = content {
        content
    } else {
        String::from("Post Not Found")
    }
}
