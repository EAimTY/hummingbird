mod db;
mod error;
mod router;

#[tokio::main]
async fn main() {
    router::init().await;
}
