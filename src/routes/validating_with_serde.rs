use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    // username: Option<String>, // agar username menjadi optional saat dimasukan di req body
    username: String,
    password: String,
}
pub async fn validate_data_with_serde(Json(body): Json<LoginRequest>) -> Json<LoginRequest> {
    // dbg!(body);
    Json(body) // deserialize terjadi disini saat mengubah struct jadi json ke response yang ada di thunder client
}