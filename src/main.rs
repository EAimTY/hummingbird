mod db;
mod error;
mod router;

#[tokio::main]
async fn main() {
    let mut repo = db::Repo::new("name", "token").await;
    repo.fetch("repo")
        .await;
    repo.get_posts().await;
    router::create_router().await;
    //router::create_router().await;
}
