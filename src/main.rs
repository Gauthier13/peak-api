mod routes;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // set logging
    tracing_subscriber::fmt::init();

    // build router
    let app = Router::new().route("/", get(routes::health::health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    tracing::info!("🚀Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
