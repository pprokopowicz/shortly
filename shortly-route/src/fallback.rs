use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn fallback() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Json(json!({"error": "Route not found"})))
        .unwrap();
}
