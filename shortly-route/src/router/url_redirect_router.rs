use crate::modules::{Modules, ModulesImpl};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Redirect,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn router() -> Router {
    Router::new().route("/:param", get(redirect::<ModulesImpl>))
}

async fn redirect<M: Modules>(
    Extension(modules): Extension<Arc<M>>,
    Path(param): Path<String>,
) -> Result<Redirect, StatusCode> {
    let entry = modules
        .url_entry_use_case()
        .execute(&param)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&entry.target_url))
}
