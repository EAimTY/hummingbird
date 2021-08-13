use axum::{
    extract::Path,
};

pub async fn post(
    Path(name): Path<String>,
) -> String {
    name
}
