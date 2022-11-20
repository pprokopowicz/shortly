use serde_json::json;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

pub async fn fallback() -> impl IntoResponse {
    let json = Json(json!({"error": "Not found"})).to_string();

    Response::builder()
        .header("Content-Type", "application/json")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(json))
        .unwrap()
}
