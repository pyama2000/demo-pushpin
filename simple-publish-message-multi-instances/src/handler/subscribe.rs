use axum::http::HeaderMap;

const CHANNEL_NAME: &str = "test";

/// Tell Grip Proxy (like Pushpin) to hold the HTTP connection open
pub async fn stream() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/event-stream".parse().unwrap());
    headers.insert("Grip-Hold", "stream".parse().unwrap());
    headers.insert("Grip-Channel", CHANNEL_NAME.parse().unwrap());
    headers
}
