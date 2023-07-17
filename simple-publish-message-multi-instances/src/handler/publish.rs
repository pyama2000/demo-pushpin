use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{config::Config, CHANNEL_NAME};

/// Publish data to receivers
pub async fn publish(State(config): State<Arc<Config>>, json: axum::Json<GripPublishJson>) {
    let GripPublishFormat::HttpStream { content } = &json.items[0].formats;
    let req = GripPublishJson::new(CHANNEL_NAME, content);
    for proxy in &config.grip_proxies {
        reqwest::Client::new()
            .post(format!(
                "http://{}:{}/publish",
                &proxy.host, &proxy.publish_port
            ))
            .json(&req)
            .send()
            .await
            .unwrap();
    }
}

#[derive(Deserialize, Serialize)]
pub struct GripPublishJson {
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
