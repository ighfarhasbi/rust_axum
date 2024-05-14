use axum::{routing::{get, post}, Router};
mod handler;
mod extrc_str;
pub fn router() -> Router {
    Router::new()
    .route("/", get(handler::hallo))
    .route("/fn1", get(handler::fungsi1))
    .route("/plain_text", get(handler::plain_text))
    .route("/json", get(handler::json))
    .route("/extrc_str", post(extrc_str::extract_str))
}
