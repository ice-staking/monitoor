mod routes;
mod errors;
use autometrics::{autometrics, prometheus_exporter};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[autometrics]
pub async fn check_status() -> Result<String, (StatusCode, String)> {
    Ok("ok".to_string())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .init();
    prometheus_exporter::init();
    let app = Router::new()
        .route("/ping", get(check_status))
        .route(
            "/metrics",
            get(|| async { prometheus_exporter::encode_http_response() }),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
