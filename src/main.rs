use axum::{routing::get, Json, Router};
use serde::Serialize;

use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tokio::main]
async fn main() {
    // set logging
    tracing_subscriber::fmt::init();

    // build router
    let app = Router::new().route("/", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    tracing::info!("🚀Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
