//! Step 8 REST API Library
//!
//! Exports modules for use by binaries (main server and export_openapi)

use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;

pub mod models;
pub mod handlers;
pub mod state;
pub mod openapi;

use openapi::ApiDoc;
use state::AppState;

/// Create the application router for testing
pub fn create_app() -> Router {
    let app_state = AppState::new();
    
    Router::new()
        .route("/health", get(health_check))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", handlers::routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Union-Find API is healthy")
}
