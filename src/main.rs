use axum::body::Body;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use example_prometheus_rust::metrics;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub registry: Registry,
}

pub async fn metrics_handler(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().await;
    let mut buffer = String::new();
    encode(&mut buffer, &state.registry).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )
        .body(Body::from(buffer))
        .unwrap()
}

#[tokio::main]
async fn main() {
    // Create the registry
    let mut state = AppState {
        registry: Registry::default(),
    };

    // BEGIN metrics section

    // Service Reset Metrics
    let srm = metrics::ServiceResetMetrics::default();
    state.registry.register(srm.name, srm.help, srm.metric);

    // END metrics section

    // Router stuff
    let state = Arc::new(Mutex::new(state));
    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state);
    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
