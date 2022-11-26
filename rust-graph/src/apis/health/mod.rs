use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct DummyConfig{

}

pub fn serve_health_apis(mut app : Router) -> Router {
    app.
    route("/ready", get(ready)).
    route("/health", get(healthy)).
    route("/config", get(config))
}

async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "")
}

async fn healthy() -> impl IntoResponse {
    (StatusCode::OK, "")
}

async fn config() -> impl IntoResponse {
    (StatusCode::OK, "")
}