use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParam {
    message: String,
    id: i32
}
pub async fn query_param(Query(query): Query<QueryParam>) -> Json<QueryParam> {
    Json(query)
}