use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn return_201() -> Response {
    (StatusCode::CREATED, "message success create from fn 201!").into_response()
    // (StatusCode::BAD_REQUEST, "punten bad request bos!").into_response()
}