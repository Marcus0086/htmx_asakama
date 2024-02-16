mod api;
mod state;
mod templates;

use api::BackendAPI;
use axum::{response::IntoResponse, routing::get, Router};

use templates::{HtmlTemplate, IndexTemplate};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let backend = BackendAPI::new();

    let assets_path = std::env::current_dir().unwrap();
    let router = Router::new()
        .route("/", get(index_page))
        .nest("/api", backend.api_router)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    info!("router initialized, now listening on port {}", "8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn index_page() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
