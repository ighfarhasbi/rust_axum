use axum::{
    extract::Request, 
    http::StatusCode, 
    middleware::Next, 
    response::Response
};

use super::read_middleware_custom::HeaderMessage;

pub async fn set_middleware_custom_header(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    // do something with `request`...
    let headers = request.headers();
    let message = headers
        .get("x-message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str().map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extension = request.extensions_mut();

    extension.insert(HeaderMessage(message));

    let response = next.run(request).await;
    Ok(response)
}