use axum::extract::Path;

pub async fn path_handler(Path(id): Path<i32>) -> String {
    id.to_string()
    // Path digunakan untuk mengambil var dari url endpoint (dlm hal ini "/path_handler/:id")
}