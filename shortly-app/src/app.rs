use anyhow::Result;
use axum::{Extension, Router, handler::Handler};
use shortly_data::{db::DbConnection, repository::url_repository::UrlEntryRepositoryImpl};
use shortly_domain::{
    mapper::url_entry_mapper::NewUrlEntryMapperImpl,
    use_case::{
        all_url_entries_use_case::AllUrlEntriesUseCase,
        create_url_entry_use_case::CreateUrlEntryUseCase, url_entry_use_case::UrlEntryUseCase,
    },
};
use shortly_route::{
    fallback,
    modules::ModulesImpl,
    router::{url_entry_router, url_redirect_router},
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn start() -> Result<()> {
    setup_tracing();

    let modules = Arc::new(modules().await?);

    let app = Router::new()
        .merge(url_entry_router::router())
        .merge(url_redirect_router::router())
        .fallback(fallback::fallback.into_service())
        .layer(Extension(modules))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let _ = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn modules() -> Result<ModulesImpl> {
    let conn = DbConnection::new().await?;

    let repository = Arc::new(UrlEntryRepositoryImpl::new(conn));

    let mapper = NewUrlEntryMapperImpl::new();
    let all_url_entries_use_case = AllUrlEntriesUseCase::new(repository.clone());
    let create_url_entry_use_case = CreateUrlEntryUseCase::new(repository.clone(), mapper);
    let url_entry_use_case = UrlEntryUseCase::new(repository.clone());

    let modules = ModulesImpl::new(
        all_url_entries_use_case,
        create_url_entry_use_case,
        url_entry_use_case,
    );

    Ok(modules)
}
