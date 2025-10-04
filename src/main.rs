mod routes;

use axum::{routing::get, Router};
use std::net::SocketAddr;

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    // set logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // build router
    let app = Router::new()
        .route("/", get(routes::health::health_check))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    tracing::info!("🚀Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
