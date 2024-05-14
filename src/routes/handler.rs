use axum::Json;
use serde_json::{Value, json};

pub async fn hallo() -> String {
    "Hello World!!, Indonesia".to_owned()
}

pub async fn fungsi1() -> String {
    "Fungsi 1, Edit".to_owned()
}

pub async fn plain_text() -> &'static str {
    "foo"
}

pub async fn json() -> Json<Value> {
    Json(json!(
        { "data": [42,55],
        "response code": "200",
        "response message": {
            "message": "Welcome to learn test",
            "about": "Lightweight Rest API Client for VSCode",
            "createdBy": "HAsbi",
            "launched": 2024
        }
     }
    ))
}
