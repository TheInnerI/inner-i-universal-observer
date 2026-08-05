//! Inner I Observer Node — Local enforcement engine.
//!
//! The Observer Node runs on the user's machine (desktop, server, edge device)
//! and enforces capabilities, observes agent actions, generates residuals,
//! records consequences, and produces signed receipts.

mod db;
mod state;
mod api;

use std::net::SocketAddr;
use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "observer_node=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Inner I Observer Node v{} starting...", env!("CARGO_PKG_VERSION"));

    let db_path = std::env::var("OBSERVER_DB_PATH")
        .unwrap_or_else(|_| "data/observer.db".to_string());

    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let pool = db::init_db(&db_path).await?;
    tracing::info!("Database initialized at {}", db_path);

    let state = AppState::new(pool).await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(api::routes::create_router())
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7411));
    tracing::info!("Observer Node listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
