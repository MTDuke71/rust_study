//! REST API Example for Mission 10
//!
//! This example demonstrates how to expose the Union-Find data structure
//! as a RESTful API using Axum and Utoipa.

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod errors;
mod handlers;
mod models;
mod openapi;
mod state;

use openapi::ApiDoc;
use state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rest_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🚀 Starting Union-Find REST API...");

    // Initialize state
    let app_state = AppState::new();

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", handlers::routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Server listening on http://{}", addr);
    println!("📚 Swagger UI: http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Union-Find API is healthy")
}
