use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Payload {
    feature: Vec<String>,
    homepage: String
}
#[derive(Serialize)]
pub struct Data {
    message: String,
    id: i32,
    success: bool,
    payload: Payload
}
pub async fn serialize_json() -> Json<Data> {
    let data = Data{
        message: "hai this message from serialize json".to_owned(),
        id: 1,
        success: true,
        payload: Payload{
            feature: vec!["feature 1".to_owned()],
            homepage: "https://google.com".to_owned()
        },
    };
    Json(data) // ambil data dari var data dan menterjemahkannya ke json data dan ditampilkan di response saat hit
}