use axum::http::StatusCode;

pub mod publish;
pub mod subscribe;

pub async fn healthz() -> StatusCode {
    StatusCode::OK
}
