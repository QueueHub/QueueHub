use log::{info, trace, warn};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub fn queue_router() -> Router {
    Router::new()
        .route("/health", get(health_checker))
        .route("/:name", post(post_queue_handler))
}

pub async fn post_queue_handler() -> impl IntoResponse {}

pub async fn health_checker() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub fn queue_handler() {
    info!("in queue handler");
}
