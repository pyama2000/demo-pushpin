use axum::{
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

const CHANNEL_NAME: &str = "test";

#[derive(Deserialize, Serialize)]
struct GripPublishJson {
    items: Vec<GripPublishItem>,
}

impl GripPublishJson {
    fn new(channel_name: impl Into<String>, content: impl Into<String>) -> Self {
        let item = GripPublishItem::new(channel_name, content);
        Self { items: vec![item] }
    }
}

#[derive(Deserialize, Serialize)]
struct GripPublishItem {
    channel: String,
    formats: GripPublishFormat,
}

impl GripPublishItem {
    fn new(channel_name: impl Into<String>, content: impl Into<String>) -> Self {
        let formats = GripPublishFormat::new(content);
        Self {
            channel: channel_name.into(),
            formats,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum GripPublishFormat {
    HttpStream { content: String },
}

impl GripPublishFormat {
    fn new(content: impl Into<String>) -> Self {
        Self::HttpStream {
            content: content.into(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/healthz", get(|| async { StatusCode::OK }))
        .route("/stream", get(stream))
        .route("/publish", post(publish));

    let port = std::env::var("REPLACE_MOCK_BACKEND_PORT")
        .unwrap_or_else(|_| panic!("REPLACE_MOCK_BACKEND_PORT must be set"));
    let addr = format!("0.0.0.0:{}", port).parse()?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Tell Grip Proxy (like Pushpin) to hold the HTTP connection open
async fn stream() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/event-stream".parse().unwrap());
    headers.insert("Grip-Hold", "stream".parse().unwrap());
    headers.insert("Grip-Channel", CHANNEL_NAME.parse().unwrap());
    headers
}

/// Publish data to receivers
async fn publish(json: axum::Json<GripPublishJson>) {
    let grip_proxy_host = std::env::var("GRIP_PROXY_HOST_NAME")
        .unwrap_or_else(|_| panic!("GRIP_PROXY_HOST_NAME must be set"));
    let grip_proxy_port = std::env::var("GRIP_PROXY_PUBLISH_PORT")
        .unwrap_or_else(|_| panic!("GRIP_PROXY_PUBLISH_PORT must be set"));

    let GripPublishFormat::HttpStream { content } = &json.items[0].formats;
    let req = GripPublishJson::new(CHANNEL_NAME, content);
    reqwest::Client::new()
        .post(format!(
            "http://{}:{}/publish",
            grip_proxy_host, grip_proxy_port
        ))
        .json(&req)
        .send()
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .unwrap_or_else(|e| panic!("failed to install Ctrl+C handler: {}", e));
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .unwrap_or_else(|e| panic!("failed to install singal handler: {}", e))
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
