mod routers;
mod error;

#[tokio::main]
async fn main() {
    routers::router().await;
}
