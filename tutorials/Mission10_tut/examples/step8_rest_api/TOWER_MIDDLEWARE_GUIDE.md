# Tower & Tower-HTTP Middleware Guide

**Mission 10 - Infrastructure Reference**  
**Topic**: Understanding Tower middleware and HTTP layers in Axum applications  
**Audience**: Developers building production-ready REST APIs

---

## 🎯 **Overview**

This document explains the **Tower** and **Tower-HTTP** libraries that form the middleware foundation of Axum web applications. These libraries enable composable, type-safe middleware for cross-cutting concerns like CORS, logging, compression, and rate limiting.

**Key Concepts**:
- Tower's `Service` trait abstraction
- Middleware composition with layers
- HTTP-specific middleware (CORS, tracing, compression)
- Production deployment considerations

---

## 📚 **Table of Contents**

1. [What is Tower?](#what-is-tower)
2. [The Service Trait](#the-service-trait)
3. [Middleware Composition](#middleware-composition)
4. [Tower-HTTP: HTTP-Specific Middleware](#tower-http-http-specific-middleware)
5. [Common Middleware Layers](#common-middleware-layers)
6. [Integration with Axum](#integration-with-axum)
7. [Production Configuration](#production-configuration)
8. [Troubleshooting](#troubleshooting)

---

## 🏗️ **What is Tower?**

**Tower** is a library for building **modular, composable network services** in Rust. It provides a framework-agnostic abstraction for middleware that works with any async Rust application.

### **Core Philosophy**

Tower treats network services as **transformable pipelines**:

```
Request → [Middleware Layer 1] → [Middleware Layer 2] → Handler → Response
```

Each layer can:
- ✅ Inspect and modify requests before they reach the handler
- ✅ Inspect and modify responses before they return to the client
- ✅ Short-circuit the pipeline (e.g., return 401 Unauthorized early)
- ✅ Add metadata (e.g., request ID, timing information)

### **Why Tower Matters for Axum**

Axum is **built on top of Tower**, which means:
- All Axum handlers are Tower `Service` implementations
- You can use any Tower middleware with Axum
- Middleware composition is type-safe and zero-cost
- The entire ecosystem (Hyper, Tonic, Axum) shares the same abstraction

### **Ecosystem Stack**

```
┌─────────────────────────────────────────────┐
│         Your Application Layer              │
│    (Handlers: create_instance, etc.)        │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│           Axum Framework                    │
│   (Routing, extractors, responses)          │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│        Tower-HTTP Middleware                │
│   (CORS, tracing, compression, etc.)        │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│         Tower Core (Service)                │
│      (Middleware composition)               │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│      Tokio (Async Runtime)                  │
│   (Event loop, TCP listeners)               │
└─────────────────────────────────────────────┘
```

---

## 🔧 **The Service Trait**

Tower's core abstraction is the `Service` trait:

```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

### **Breaking Down the Trait**

**Associated Types**:
- `Response` - What the service produces on success
- `Error` - What the service produces on failure
- `Future` - The async computation that produces `Result<Response, Error>`

**Methods**:
- `poll_ready()` - Check if service can accept a request (backpressure)
- `call()` - Process the request and return a future

### **Simple Service Example**

```rust
use tower::Service;
use std::future::{ready, Ready};

struct HelloService;

impl Service<String> for HelloService {
    type Response = String;
    type Error = String;
    type Future = Ready<Result<String, String>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))  // Always ready
    }

    fn call(&mut self, name: String) -> Self::Future {
        ready(Ok(format!("Hello, {}!", name)))
    }
}

// Usage:
let mut service = HelloService;
let response = service.call("World".to_string()).await?;
// response == "Hello, World!"
```

### **Why Service Matters**

The `Service` trait enables:
1. **Composability** - Services can wrap other services
2. **Abstraction** - HTTP server, cache, load balancer all implement `Service`
3. **Type Safety** - Compiler ensures middleware compatibility
4. **Testability** - Mock services for unit testing

---

## 🧩 **Middleware Composition**

Tower uses **layers** to compose middleware. Layers wrap services to add functionality.

### **Without Tower: Manual Middleware**

```rust
async fn handler(req: Request) -> Response {
    // Log the request
    tracing::info!("Request received: {:?}", req);
    
    // Check CORS
    if !is_cors_valid(&req) {
        return Response::builder()
            .status(403)
            .body("CORS error".into())
            .unwrap();
    }
    
    // Add request ID
    let request_id = Uuid::new_v4();
    
    // Process request
    let mut response = process_request(req).await;
    
    // Add response headers
    response.headers_mut().insert("X-Request-ID", request_id.into());
    
    // Log response
    tracing::info!("Response sent: status={}", response.status());
    
    response
}
```

**Problems**:
- ❌ Every handler repeats boilerplate
- ❌ Hard to test individual concerns
- ❌ Error-prone (easy to forget steps)
- ❌ Can't reuse across handlers

### **With Tower: Composable Layers**

```rust
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::request_id::{SetRequestIdLayer, PropagateRequestIdLayer};

let app = Router::new()
    .route("/api/v1/unionfind", post(create_instance))
    .layer(TraceLayer::new_for_http())           // Logging
    .layer(CorsLayer::permissive())              // CORS
    .layer(SetRequestIdLayer::new_from_uuid())   // Request ID
    .layer(PropagateRequestIdLayer::new("X-Request-ID")); // Add to response
```

**Benefits**:
- ✅ Declarative and readable
- ✅ Each layer is independent and reusable
- ✅ Type-safe composition (compiler checks compatibility)
- ✅ Zero-cost abstractions (no runtime overhead)

### **Layer Ordering**

Layers are applied **inside-out**:

```rust
.layer(Layer1)  // Outermost (runs first on request, last on response)
.layer(Layer2)
.layer(Layer3)  // Innermost (runs last on request, first on response)
```

**Request flow**: Layer1 → Layer2 → Layer3 → Handler  
**Response flow**: Handler → Layer3 → Layer2 → Layer1

**Example**:
```rust
.layer(TraceLayer::new_for_http())    // 1. Log request → 4. Log response
.layer(CorsLayer::permissive())       // 2. Check CORS → 3. Add CORS headers
.layer(TimeoutLayer::new(...))        // 3. Start timeout → 2. Check timeout
// Handler runs here                   // 4. Process request
```

---

## 🌐 **Tower-HTTP: HTTP-Specific Middleware**

**tower-http** provides HTTP-focused middleware layers built on Tower's `Service` abstraction.

### **Cargo.toml Configuration**

```toml
[dependencies]
tower-http = { version = "0.5", features = ["cors", "trace", "compression-full"] }
```

**Available Features**:
- `cors` - Cross-Origin Resource Sharing
- `trace` - Request/response tracing
- `compression-full` - Gzip, Brotli, Deflate compression
- `timeout` - Request timeouts
- `limit` - Request size limits
- `validate-request` - Header/body validation
- `set-header` - Add/modify headers
- `propagate-header` - Copy headers from request to response

### **CORS (Cross-Origin Resource Sharing)**

**Problem**: Browsers block requests from different origins (different domain, port, or protocol).

**Example**: Frontend at `http://localhost:3000` can't call API at `http://localhost:8080` without CORS headers.

**Solution**:

```rust
use tower_http::cors::{CorsLayer, Any};
use http::{Method, HeaderValue};

// Permissive (allow all origins - development only!)
let cors = CorsLayer::permissive();

// Production (restrict origins)
let cors = CorsLayer::new()
    .allow_origin("https://myapp.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::DELETE])
    .allow_headers(Any)
    .max_age(Duration::from_secs(3600));

let app = Router::new()
    .route("/api/v1/unionfind", post(create_instance))
    .layer(cors);
```

**What it adds to responses**:
```http
Access-Control-Allow-Origin: https://myapp.com
Access-Control-Allow-Methods: GET, POST, DELETE
Access-Control-Max-Age: 3600
```

### **Tracing (Logging)**

**Problem**: Need to log all HTTP requests/responses for debugging and monitoring.

**Solution**:

```rust
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::Level;

let trace = TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new()
        .level(Level::INFO)
        .include_headers(true))
    .on_response(DefaultOnResponse::new()
        .level(Level::INFO)
        .include_headers(true));

let app = Router::new()
    .route("/api", get(handler))
    .layer(trace);
```

**Integration with tracing-subscriber**:

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "step8_rest_api=debug,tower_http=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();
```

**Output**:
```
INFO request{method=POST path="/api/v1/unionfind" version=HTTP/1.1}: tower_http::trace::on_request: started processing request
INFO request{method=POST path="/api/v1/unionfind"}: tower_http::trace::on_response: finished processing request status=201 latency=5ms
```

### **Compression**

**Problem**: Reduce bandwidth by compressing responses.

**Solution**:

```rust
use tower_http::compression::CompressionLayer;

let app = Router::new()
    .route("/api", get(handler))
    .layer(CompressionLayer::new());
```

**What it does**:
- Automatically compresses responses based on `Accept-Encoding` header
- Supports gzip, brotli, deflate
- Only compresses if client supports it

**Headers added**:
```http
Content-Encoding: gzip
Vary: Accept-Encoding
```

---

## 📋 **Common Middleware Layers**

### **Complete Middleware Reference**

| **Middleware** | **Feature Flag** | **Purpose** | **Use Case** |
|----------------|------------------|-------------|--------------|
| `CorsLayer` | `cors` | Cross-origin requests | Frontend calling API from different origin |
| `TraceLayer` | `trace` | Request/response logging | Monitor API usage, debug issues |
| `CompressionLayer` | `compression-full` | Compress responses | Reduce bandwidth usage |
| `TimeoutLayer` | `timeout` | Request timeouts | Prevent hanging requests |
| `RequestBodyLimitLayer` | `limit` | Limit request size | Prevent DoS attacks |
| `ValidateRequestHeaderLayer` | `validate-request` | Validate headers | Require auth tokens, API keys |
| `SetRequestHeaderLayer` | `set-header` | Add request headers | Add internal metadata |
| `SetResponseHeaderLayer` | `set-header` | Add response headers | Add security headers |
| `PropagateHeaderLayer` | `propagate-header` | Copy headers | Propagate request ID |
| `SensitiveHeadersLayer` | `set-header` | Redact sensitive headers | Hide Authorization in logs |

### **Example: Production Middleware Stack**

```rust
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
};
use http::{header, HeaderValue};
use std::time::Duration;

let app = Router::new()
    .route("/api/v1/unionfind", post(create_instance))
    // Security headers
    .layer(SetSensitiveRequestHeadersLayer::new([header::AUTHORIZATION]))
    // CORS (restrict to production domain)
    .layer(CorsLayer::new()
        .allow_origin("https://myapp.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE]))
    // Logging
    .layer(TraceLayer::new_for_http())
    // Compression
    .layer(CompressionLayer::new())
    // Timeout (30 seconds)
    .layer(TimeoutLayer::new(Duration::from_secs(30)))
    // Request size limit (10 MB)
    .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
    .with_state(app_state);
```

---

## 🔗 **Integration with Axum**

### **In Mission 10 Step 8 REST API**

**File: `Cargo.toml`**
```toml
[dependencies]
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
```

**File: `src/main.rs`**
```rust
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "step8_rest_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🚀 Starting Union-Find REST API...");

    let app_state = AppState::new();

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", handlers::routes())
        .layer(CorsLayer::permissive())  // ← Tower-HTTP CORS
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server listening on http://{}", addr);
    println!("📚 Swagger UI: http://{}/swagger-ui", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### **Why These Specific Features?**

**`cors`**: 
- Enables frontend apps (React, Vue, Angular) to call the API
- Essential for development (different ports) and production (different domains)
- In our case: Swagger UI runs in browser and needs to make API calls

**`trace`**:
- Integrates with `tracing` and `tracing-subscriber` for structured logging
- Logs every HTTP request with method, path, status, latency
- Essential for debugging and monitoring production systems

---

## 🚀 **Production Configuration**

### **Development vs. Production**

**Development (permissive)**:
```rust
let cors = CorsLayer::permissive();  // Allow all origins
let timeout = TimeoutLayer::new(Duration::from_secs(300));  // 5 min timeout
```

**Production (restrictive)**:
```rust
let cors = CorsLayer::new()
    .allow_origin("https://myapp.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .max_age(Duration::from_secs(3600));

let timeout = TimeoutLayer::new(Duration::from_secs(30));  // 30 sec timeout
```

### **Environment-Based Configuration**

```rust
use std::env;

let cors = if env::var("ENVIRONMENT").unwrap_or_default() == "production" {
    CorsLayer::new()
        .allow_origin(env::var("ALLOWED_ORIGIN")
            .unwrap_or("https://myapp.com".to_string())
            .parse::<HeaderValue>()
            .unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
} else {
    CorsLayer::permissive()
};

let app = Router::new()
    .route("/api", post(handler))
    .layer(cors);
```

### **Security Headers**

Add security headers for production:

```rust
use tower_http::set_header::SetResponseHeaderLayer;
use http::header;

let app = Router::new()
    .route("/api", get(handler))
    // Security headers
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    ))
    .layer(SetResponseHeaderLayer::if_not_present(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    ));
```

---

## 🐛 **Troubleshooting**

### **Issue: CORS Errors in Browser**

**Symptom**:
```
Access to fetch at 'http://localhost:8080/api/v1/unionfind' from origin 
'http://localhost:3000' has been blocked by CORS policy
```

**Fix**: Add `CorsLayer::permissive()` for development:
```rust
.layer(CorsLayer::permissive())
```

**Production Fix**: Allow specific origin:
```rust
.layer(CorsLayer::new()
    .allow_origin("https://myapp.com".parse::<HeaderValue>().unwrap()))
```

### **Issue: Tracing Not Showing**

**Symptom**: No request/response logs appear.

**Fix 1**: Initialize `tracing-subscriber`:
```rust
tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new("tower_http=debug"))
    .with(tracing_subscriber::fmt::layer())
    .init();
```

**Fix 2**: Set `RUST_LOG` environment variable:
```bash
RUST_LOG=tower_http=debug cargo run
```

### **Issue: Compilation Errors with Features**

**Symptom**:
```
error[E0433]: failed to resolve: could not find `compression` in `tower_http`
```

**Fix**: Enable required feature:
```toml
tower-http = { version = "0.5", features = ["compression-full"] }
```

### **Issue: Layer Order Matters**

**Problem**: Timeout layer after compression causes premature timeouts.

**Bad**:
```rust
.layer(CompressionLayer::new())    // Compression takes time
.layer(TimeoutLayer::new(...))     // Timeout starts before compression!
```

**Good**:
```rust
.layer(TimeoutLayer::new(...))     // Timeout wraps everything
.layer(CompressionLayer::new())    // Compression happens inside timeout
```

---

## 📚 **Additional Resources**

### **Official Documentation**
- [Tower Documentation](https://docs.rs/tower/)
- [Tower-HTTP Documentation](https://docs.rs/tower-http/)
- [Axum Middleware Guide](https://docs.rs/axum/latest/axum/middleware/index.html)

### **Examples**
- [Tower Examples](https://github.com/tower-rs/tower/tree/master/tower/examples)
- [Tower-HTTP Examples](https://github.com/tower-rs/tower-http/tree/main/tower-http/examples)
- [Axum Middleware Examples](https://github.com/tokio-rs/axum/tree/main/examples/middleware)

### **Related Concepts**
- [[service-trait-pattern]] - Understanding Tower's `Service` abstraction
- [[middleware-composition]] - Building composable middleware
- [[axum-extractors]] - How extractors work with Tower services
- [[production-deployment]] - Deploying Rust web services

---

## ✅ **Summary**

### **Tower Core**
- ✅ Provides `Service` trait for composable network services
- ✅ Enables type-safe middleware composition via layers
- ✅ Zero-cost abstractions (no runtime overhead)
- ✅ Framework-agnostic (works with Axum, Hyper, Tonic)

### **Tower-HTTP**
- ✅ HTTP-specific middleware built on Tower
- ✅ Common layers: CORS, tracing, compression, timeouts
- ✅ Production-ready security and performance features
- ✅ Seamless integration with Axum

### **In Mission 10**
- ✅ `tower` = Core middleware framework (required by Axum)
- ✅ `tower-http` = HTTP layers for CORS, tracing
- ✅ Enables production-ready REST API deployment
- ✅ Complements OpenAPI documentation (REQ-10)

---

**Document Status**: Reference Guide  
**Last Updated**: December 24, 2025  
**Related Guides**: [DAY8_ARCHITECTURE_GUIDE.md](DAY8_ARCHITECTURE_GUIDE.md), [DAY11_IMPLEMENTATION_GUIDE.md](DAY11_IMPLEMENTATION_GUIDE.md)
