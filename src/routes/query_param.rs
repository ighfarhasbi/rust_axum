use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] // Deserialize untuk mengambil var dari queryparam untuk di proses di codingan ini, Serialize untuk mentransfer dari kodingan ini ke json yang ada di response saat hit
pub struct QueryParam {
    message: String,
    id: i32
}
pub async fn query_param(Query(query): Query<QueryParam>) -> Json<QueryParam> {
    Json(query)
    // jika ada var yang harus diabil dari url, cara mengambilnya ada dengan perintah Query<QueryParam>,
    // untuk membuat parameter query bisa dipanggil (dlm hal ini diterjemahkan menjadi Json(query)).
}