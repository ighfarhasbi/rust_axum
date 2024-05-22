use axum::http::{header, HeaderMap};

pub async fn user_agent(headers: HeaderMap) -> String {
    if let Some(user_agent_header) = headers.get(header::USER_AGENT) {
        return user_agent_header.to_str().unwrap_or_default().to_owned(); // untuk mengambil header yang digunakan saat hit dan ditampilkan di halaman response
    }
    "No user agent header found".to_owned()
}