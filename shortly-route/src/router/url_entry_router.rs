use crate::model::url_entry::{NewUrlEntry, UrlEntry};
use crate::modules::{Modules, ModulesImpl};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/entries",
            get(all_entries::<ModulesImpl>).post(create_entry::<ModulesImpl>),
        )
        .route(
            "/api/entries/:param",
            get(entry_by_external_id::<ModulesImpl>),
        )
}

async fn all_entries<M: Modules>(
    Extension(modules): Extension<Arc<M>>,
) -> Result<impl IntoResponse, StatusCode> {
    let entries = modules
        .all_url_entries_use_case()
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|entry| UrlEntry::from(entry))
        .collect::<Vec<UrlEntry>>();

    Ok(Json(entries))
}

async fn entry_by_external_id<M: Modules>(
    Extension(modules): Extension<Arc<M>>,
    Path(param): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let entry = modules
        .url_entry_use_case()
        .execute(&param)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = UrlEntry::from(entry);

    Ok(Json(data))
}

async fn create_entry<M: Modules>(
    Extension(modules): Extension<Arc<M>>,
    Json(data): Json<NewUrlEntry>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_entry = modules
        .create_url_entry_use_case()
        .execute(&data.target_url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = UrlEntry::from(new_entry);

    Ok(Json(data))
}
