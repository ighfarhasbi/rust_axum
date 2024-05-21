use axum::http::HeaderMap;

pub async fn custom_header(header: HeaderMap) -> String{
    let message_val = header.get("x-message").unwrap();
    let message = message_val.to_str().unwrap();
    message.to_owned()
}