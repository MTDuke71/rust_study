//! Step 8: REST API with OpenAPI/Swagger
//!
//! **Learning Objective**: Build a production-ready REST API for Union-Find operations.
//!
//! **Key Concepts**:
//! - RESTful API design
//! - Async web server with Axum
//! - OpenAPI documentation with utoipa
//! - State management in async Rust

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use step8_rest_api::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "step8_rest_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🚀 Starting Union-Find REST API...");

    // Build router using shared function
    let app = create_app();

    // Get host and port from environment or use defaults
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    println!("✅ Server listening on http://{}", addr);
    println!("📚 Swagger UI: http://{}/swagger-ui", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
