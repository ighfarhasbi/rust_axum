mod handler;
mod extrc_str;
mod body_json;
mod path_handler;
mod query_param;
mod mirror_user_agent;
mod custom_header;
mod middleware_mirror;
mod read_middleware_custom;
mod set_middleware_custom;
mod always_error;
mod return_201;
mod serialize_json;
mod validating_with_serde;
mod validating_custom_extractor;

use axum::{http::Method, middleware, routing::{get, post}, Extension, Router};
use tower_http::cors::{Any, CorsLayer};

use self::set_middleware_custom::set_middleware_custom_header;

#[derive(Clone)]

pub struct ShareData {
    pub message: String
}

pub fn router() -> Router {
    let cors = CorsLayer::new()
    .allow_origin(Any)
        .allow_methods(vec![Method::GET])
        .allow_headers(Any)
        .allow_credentials(false);

    let share_data = ShareData {
        message: "hai this message from share data middleware".to_owned()
    };

    Router::new()
    .route("/custom_middleware", get(read_middleware_custom::read_middleware_custom_header))
    .route("/", get(handler::hallo)) // rout ini akan terkena error jika tidak ada header "x-message" karena middleware dibaris berikutnya
    .layer(middleware::from_fn(set_middleware_custom_header))
    .route("/fn1", get(handler::fungsi1))
    .route("/plain_text", get(handler::plain_text))
    .route("/json", get(handler::json))
    .route("/extrc_str", post(extrc_str::extract_str))
    .route("/body_json", post(body_json::mirror_body_json))
    .route("/path_handler/:id", get(path_handler::path_handler))
    .route("/query_param", get(query_param::query_param))
    .route("/user_agent", get(mirror_user_agent::user_agent))
    .route("/custom_header", get(custom_header::custom_header))
    .route("/middleware", get(middleware_mirror::middleware_message))
    .route("/always_error", get(always_error::always_error))
    .route("/return_201", post(return_201::return_201))
    .route("/serialize_json", get(serialize_json::serialize_json))
    .route("/validate_data", post(validating_with_serde::validate_data_with_serde))
    .route("/custom_validator", post(validating_custom_extractor::custom_validating_extractor))
    .layer(cors)
    .layer(Extension(share_data))
    

}
