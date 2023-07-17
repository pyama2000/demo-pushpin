use std::sync::Arc;

use axum::routing::{get, post};

mod config;
mod handler;

const CHANNEL_NAME: &str = "test";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(config::Config::from_env());
    let app = axum::Router::new()
        .route("/healthz", get(handler::healthz))
        .route("/stream", get(handler::subscribe::stream))
        .route("/publish", post(handler::publish::publish))
        .with_state(config.clone());

    let addr = format!("0.0.0.0:{}", &config.port).parse()?;

    tracing::info!("listening on :{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
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
        _ = ctrl_c => tracing::debug!("receive ctrl_c signal"),
        _ = terminate => tracing::debug!("receive terminate"),
    }

    tracing::info!("signal received, starting graceful shutdown");
}
