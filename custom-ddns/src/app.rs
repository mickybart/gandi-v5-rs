mod cddns;
mod health;

use std::{error::Error, sync::Arc};

use axum::{routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::config::AppConfig;

pub(crate) async fn run_app() -> Result<(), Box<dyn Error>> {
    observability::init();

    let shared_config = Arc::new(AppConfig::build()?);

    let app = Router::new()
        .route("/health", get(health::health))
        .route(
            "/gandi/:fqdn/:rrset_name/:rrset_type/:rrset_value",
            get(cddns::gandi),
        )
        .with_state(Arc::clone(&shared_config))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind(&shared_config.listen).await?;

    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

mod observability {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    pub(crate) fn init() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}
