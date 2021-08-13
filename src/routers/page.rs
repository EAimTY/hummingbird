use axum::{
    extract::Path,
};

pub async fn page(
    Path(name): Path<String>,
) -> String {
    name
}
