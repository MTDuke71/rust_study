//! Step 8: REST API with OpenAPI/Swagger
//!
//! **Learning Objective**: Build a production-ready REST API for Union-Find operations.
//!
//! **Key Concepts**:
//! - RESTful API design
//! - Async web server with Axum
//! - OpenAPI documentation with utoipa
//! - State management in async Rust

use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import models and handlers
mod models;
mod handlers;
mod state;
mod errors;
mod openapi;

use openapi::ApiDoc;
use state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "step8_rest_api=debug,tower_http=debug".into()),
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server listening on http://{}", addr);
    println!("📚 Swagger UI: http://{}/swagger-ui", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Union-Find API is healthy")
}
