use axum::Json;
use serde::{Deserialize, Serialize}; 
use serde_json::Number;
#[derive(Serialize, Deserialize, Debug)] //agar rout di file mod.rs tidak error (code and decode to json)

pub struct MirrorJson {
    message: String,
    code: Number,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJson> {
    Json(body)
}