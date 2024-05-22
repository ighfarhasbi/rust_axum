use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(pub String);
pub async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
    // extension digunakan untuk membuat si parameter menjadi bertipe sesuai dengan subdependensi, 
    // dalam hal ini parameter "message" menajdi bertipe HeaderMessage (dari library dependensi) melalui perintah extension
}