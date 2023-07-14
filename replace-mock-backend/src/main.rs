use axum::{
    http::{HeaderMap, StatusCode},
    routing::get,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/healthz", get(|| async { StatusCode::OK }))
        .route("/stream", get(stream));

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
    headers.insert("Grip-Channel", "test".parse().unwrap());
    headers
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
