# Day 13: Implementing Structured Errors & Testing 🧪🔧

**Mission 10 - Day 13 Activity**  
**Goal**: Make runtime errors match the beautiful OpenAPI documentation we created on Day 12! 🎯  
**Time**: 90-120 minutes (testing is satisfying work! ☕)  
**Prerequisites**: Completed Day 12 (Comprehensive OpenAPI specification with error examples)

> **Today's Mission**: Yesterday we wrote the instruction manual. Today we make sure the product actually works as advertised! 📋✅

---

## 🎯 **Today's Success Criteria** (aka Making Promises We Can Keep)

By the end of Day 13, you'll have:
- [ ] 🏗️ **Error Implementation**: Runtime errors use semantic codes (INVALID_SIZE, not "400 Bad Request")
- [ ] 📊 **Structured Details**: Error responses include `details` and `field_errors` where appropriate
- [ ] 🧪 **Integration Tests**: Automated tests that verify OpenAPI spec accuracy
- [ ] ✅ **Spec Validation**: Runtime behavior matches documented examples
- [ ] 🎭 **Edge Case Coverage**: Tests for all error scenarios from Day 12's test matrix
- [ ] 📸 **Example Collection**: Real API responses saved as documentation
- [ ] 🔍 **Manual Testing**: Swagger UI verification of all endpoints

**The Gap We're Closing**: Day 12 documented what errors *should* look like. Day 13 makes them *actually* look that way!

---

## 🔄 **Day 12 → Day 13 Transition**

### **What We Have from Day 12** 📚
```rust
// Beautiful OpenAPI documentation:
(status = 400,
 description = "Invalid request parameters",
 body = ErrorResponse,
 example = json!({
     "code": "INVALID_SIZE",
     "message": "Size must be greater than 0",
     "details": {
         "provided": 0,
         "minimum": 1
     }
 }))
```

### **What's Actually Happening** 😅
```rust
// Generic error response:
ErrorResponse {
    code: "400 Bad Request",  // ❌ HTTP status, not semantic
    message: "Size must be at least 1"
    // No details, no field_errors
}
```

### **What We'll Build Today** ✨
```rust
// Runtime matches docs:
ErrorResponse::new(error_codes::INVALID_SIZE, "Size must be greater than 0")
    .with_details(HashMap::from([
        ("provided".into(), json!(0)),
        ("minimum".into(), json!(1))
    ]))
```

---

## 📋 **Step 1: Implement Structured Error Helpers (20 minutes)** 🛠️

### **1.1 Enhanced Error Response Functions**

**Current state**: One generic `error_response()` function  
**Goal**: Type-safe error builders that match our OpenAPI spec

Update `handlers.rs`:

```rust
use std::collections::HashMap;
use serde_json::json;
use crate::models::error_codes;

// Generic error for simple cases
fn error_response(code: StatusCode, message: &str) -> Response {
    (
        code,
        Json(ErrorResponse::new(code.to_string(), message)),
    ).into_response()
}

// Semantic error codes with details
fn error_with_code(
    status: StatusCode,
    error_code: &str,
    message: impl Into<String>,
) -> Response {
    (
        status,
        Json(ErrorResponse::new(error_code, message)),
    ).into_response()
}

// Error with structured details
fn error_with_details(
    status: StatusCode,
    error_code: &str,
    message: impl Into<String>,
    details: HashMap<String, serde_json::Value>,
) -> Response {
    (
        status,
        Json(ErrorResponse::new(error_code, message).with_details(details)),
    ).into_response()
}

// Error with field-level validation errors
fn error_with_fields(
    status: StatusCode,
    error_code: &str,
    message: impl Into<String>,
    field_errors: HashMap<String, String>,
) -> Response {
    (
        status,
        Json(ErrorResponse::new(error_code, message).with_field_errors(field_errors)),
    ).into_response()
}
```

**Why this matters**: Type-safe helpers prevent typos in error codes and ensure consistency! 🎯

---

## 📋 **Step 2: Update create_instance Handler (15 minutes)** 🏗️

### **2.1 Replace Generic Errors with Semantic Codes**

**Before** (Day 12):
```rust
if payload.size == 0 {
    return error_response(StatusCode::BAD_REQUEST, "Size must be at least 1");
}
```

**After** (Day 13):
```rust
// Validation with semantic error code
if payload.size == 0 {
    return error_with_details(
        StatusCode::BAD_REQUEST,
        error_codes::INVALID_SIZE,
        "Size must be greater than 0",
        HashMap::from([
            ("provided".into(), json!(0)),
            ("minimum".into(), json!(1)),
        ]),
    );
}

if payload.size > 100_000 {
    return error_with_details(
        StatusCode::BAD_REQUEST,
        error_codes::INVALID_SIZE,
        "Size exceeds maximum allowed",
        HashMap::from([
            ("provided".into(), json!(payload.size)),
            ("maximum".into(), json!(100_000)),
        ]),
    );
}
```

**Test it manually** 🧪:
```bash
# Start server
cargo run

# Test zero size (new terminal)
curl -X POST http://localhost:8080/api/v1/unionfind \
  -H "Content-Type: application/json" \
  -d '{"size": 0}'

# Expected response:
{
  "code": "INVALID_SIZE",
  "message": "Size must be greater than 0",
  "details": {
    "provided": 0,
    "minimum": 1
  }
}
```

---

## 📋 **Step 3: Update union_elements Handler (20 minutes)** 🔗

### **3.1 Add Instance Not Found Error**

```rust
pub async fn union_elements(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UnionRequest>,
) -> Response {
    let result = state.get_instance(id, |uf| {
        uf.union(payload.element1, payload.element2)
            .and_then(|merged| {
                uf.find(payload.element1).map(|root| (merged, root))
            })
    });

    match result {
        Some(Ok((merged, root))) => (
            StatusCode::OK,
            Json(UnionResponse { merged, root }),
        ).into_response(),
        Some(Err(e)) => {
            // Parse error message to determine specific error
            if e.contains("out of bounds") || e.contains("exceeds") {
                error_with_details(
                    StatusCode::BAD_REQUEST,
                    error_codes::ELEMENT_OUT_OF_BOUNDS,
                    e,
                    HashMap::from([
                        ("element1".into(), json!(payload.element1)),
                        ("element2".into(), json!(payload.element2)),
                    ]),
                )
            } else {
                error_with_code(StatusCode::BAD_REQUEST, error_codes::INVALID_UNION, e)
            }
        },
        None => error_with_details(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
            HashMap::from([
                ("id".into(), json!(id.to_string())),
            ]),
        ),
    }
}
```

---

## 📋 **Step 4: Update Remaining Handlers (20 minutes)** 🔄

### **4.1 find_element Handler**

```rust
match result {
    Some(Ok(root)) => (
        StatusCode::OK,
        Json(FindResponse {
            element: params.element,
            root,
        }),
    ).into_response(),
    Some(Err(e)) => error_with_details(
        StatusCode::BAD_REQUEST,
        error_codes::ELEMENT_OUT_OF_BOUNDS,
        "Element index exceeds instance size",
        HashMap::from([
            ("element".into(), json!(params.element)),
        ]),
    ),
    None => error_with_code(
        StatusCode::NOT_FOUND,
        error_codes::INSTANCE_NOT_FOUND,
        "No Union-Find instance exists with the given ID",
    ),
}
```

### **4.2 check_connected Handler**

```rust
match result {
    Some(Ok(connected)) => (
        StatusCode::OK,
        Json(ConnectedResponse { connected }),
    ).into_response(),
    Some(Err(e)) => error_with_details(
        StatusCode::BAD_REQUEST,
        error_codes::ELEMENT_OUT_OF_BOUNDS,
        "One or more element indices exceed instance size",
        HashMap::from([
            ("element1".into(), json!(params.element1)),
            ("element2".into(), json!(params.element2)),
        ]),
    ),
    None => error_with_code(
        StatusCode::NOT_FOUND,
        error_codes::INSTANCE_NOT_FOUND,
        "No Union-Find instance exists with the given ID",
    ),
}
```

### **4.3 delete_instance Handler**

```rust
pub async fn delete_instance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response {
    if state.delete_instance(id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        error_with_details(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
            HashMap::from([
                ("id".into(), json!(id.to_string())),
            ]),
        )
    }
}
```

---

## 📋 **Step 5: Integration Tests (30 minutes)** 🧪

### **5.1 Create Integration Test Suite**

Create `tests/api_tests.rs`:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::json;
use step8_rest_api::{create_app, models::ErrorResponse};

#[tokio::test]
async fn test_create_instance_invalid_size_zero() {
    let app = create_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();
    
    // Verify semantic error code
    assert_eq!(error.code, "INVALID_SIZE");
    assert!(error.message.contains("greater than 0"));
    
    // Verify structured details
    let details = error.details.unwrap();
    assert_eq!(details.get("provided").unwrap(), &json!(0));
    assert_eq!(details.get("minimum").unwrap(), &json!(1));
}

#[tokio::test]
async fn test_instance_not_found() {
    let app = create_app();
    
    let fake_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/stats", fake_uuid))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(error.code, "INSTANCE_NOT_FOUND");
}

#[tokio::test]
async fn test_element_out_of_bounds() {
    let app = create_app();
    
    // First create an instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let create_result: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let id = create_result["id"].as_str().unwrap();
    
    // Try to find element out of bounds
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/find?element=100", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(error.code, "ELEMENT_OUT_OF_BOUNDS");
    assert!(error.details.is_some());
}
```

### **5.2 Run Tests**

```bash
cargo test --test api_tests
```

**Success criteria** ✅:
- All tests pass
- Error codes match OpenAPI documentation
- Details fields contain expected data

---

## 📋 **Step 6: Swagger UI Validation (20 minutes)** 🎨

### **6.1 Manual Testing Checklist**

Start the server and test in Swagger UI:

**Create Instance Tests:**
- [ ] POST with `size: 0` → Returns `400 Bad Request` with `INVALID_SIZE` error code and details
- [ ] POST with `size: -1` → Returns `422 Unprocessable Entity` (Axum JSON deserialization error - usize cannot be negative)
  - Expected response: `"Failed to deserialize the JSON body into the target type: size: invalid value: integer `-1`, expected usize at line 2 column 12"`
  - Note: This error occurs at the framework level before reaching our handler
- [ ] POST with `size: 200000` → Returns `400 Bad Request` with `INVALID_SIZE` error code (max exceeded)

**Union Tests:**
- [ ] POST union with invalid UUID → Returns `INSTANCE_NOT_FOUND`
- [ ] POST union with element > size → Returns `ELEMENT_OUT_OF_BOUNDS` with field info
- [ ] POST union valid elements → Returns success

**Find Tests:**
- [ ] GET find with non-existent instance → Returns `INSTANCE_NOT_FOUND`
- [ ] GET find with out-of-bounds element → Returns `ELEMENT_OUT_OF_BOUNDS`

**Delete Tests:**
- [ ] DELETE non-existent instance → Returns `INSTANCE_NOT_FOUND` with ID

### **6.2 Compare Responses to Documentation**

For each error scenario:
1. Execute in Swagger UI
2. Copy actual response
3. Compare to OpenAPI example
4. Verify they match! 🎯

---

## 📋 **Step 7: Response Collection & Documentation (15 minutes)** 📸

### **7.1 Save Real Examples**

Create `examples/actual_responses.md`:

```markdown
# Actual API Responses - Day 13

## Invalid Size Error

**Request:**
```json
POST /api/v1/unionfind
{"size": 0}
```

**Response (400 Bad Request):**
```json
{
  "code": "INVALID_SIZE",
  "message": "Size must be greater than 0",
  "details": {
    "provided": 0,
    "minimum": 1
  }
}
```

## Instance Not Found

**Request:**
```
GET /api/v1/unionfind/550e8400-e29b-41d4-a716-446655440000/stats
```

**Response (404 Not Found):**
```json
{
  "code": "INSTANCE_NOT_FOUND",
  "message": "No Union-Find instance exists with the given ID"
}
```
```

---

## 📋 **Step 8: Performance Testing (Optional, 10 minutes)** ⚡

### **8.1 Error Response Performance**

Verify that structured errors don't slow things down:

```bash
# Benchmark error responses
cargo bench --bench error_responses
```

Create `benches/error_responses.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use step8_rest_api::models::{ErrorResponse, error_codes};
use std::collections::HashMap;
use serde_json::json;

fn bench_simple_error(c: &mut Criterion) {
    c.bench_function("simple_error", |b| {
        b.iter(|| {
            ErrorResponse::new(
                black_box(error_codes::INVALID_SIZE),
                black_box("Test error"),
            )
        });
    });
}

fn bench_error_with_details(c: &mut Criterion) {
    c.bench_function("error_with_details", |b| {
        b.iter(|| {
            ErrorResponse::new(error_codes::INVALID_SIZE, "Test error")
                .with_details(HashMap::from([
                    ("provided".into(), json!(0)),
                    ("minimum".into(), json!(1)),
                ]))
        });
    });
}

criterion_group!(benches, bench_simple_error, bench_error_with_details);
criterion_main!(benches);
```

---

## 📋 **Step 9: Update Models.rs Documentation (10 minutes)** 📝

### **9.1 Remove dead_code Attributes**

Now that we're actually using the error codes and helper methods, remove the `#[allow(dead_code)]`:

```rust
// Remove these attributes:
// #[allow(dead_code)]  ❌

// The compiler will now verify we're using them! ✅
```

### **9.2 Add Usage Examples to Error Codes**

Update `models.rs`:

```rust
/// Common error codes for OpenAPI documentation
///
/// # Examples
/// ```
/// use step8_rest_api::models::{ErrorResponse, error_codes};
/// 
/// let error = ErrorResponse::new(
///     error_codes::INVALID_SIZE,
///     "Size must be greater than 0"
/// );
/// ```
pub mod error_codes {
    pub const INVALID_SIZE: &str = "INVALID_SIZE";
    // ... rest of codes
}
```

---

## 📋 **Step 10: Final Validation (10 minutes)** ✅

### **10.1 Comprehensive Test Run**

```bash
# All unit tests
cargo test

# All integration tests  
cargo test --test api_tests

# Clippy (should pass with no warnings)
cargo clippy -- -D warnings

# Build release
cargo build --release

# Start server
cargo run --release
```

### **10.2 OpenAPI Spec Validation**

1. Navigate to `http://localhost:8080/swagger-ui`
2. Try each error scenario from Day 12's test matrix
3. Verify responses match documented examples
4. Export spec: `http://localhost:8080/api-docs/openapi.json`
5. Validate at https://editor.swagger.io/

**Success checklist** 🎉:
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Error codes are semantic (not HTTP status codes)
- [ ] Structured details in error responses
- [ ] Swagger UI shows correct error examples
- [ ] OpenAPI spec validates without errors
- [ ] Runtime behavior matches documentation

---

## 🎓 **What We Accomplished Today** 🏆

### **Before Day 13**:
```json
{
  "code": "400 Bad Request",
  "message": "Size must be at least 1"
}
```

### **After Day 13**:
```json
{
  "code": "INVALID_SIZE",
  "message": "Size must be greater than 0",
  "details": {
    "provided": 0,
    "minimum": 1
  }
}
```

### **Key Improvements**:
1. ✅ **Semantic Error Codes**: Machine-readable codes for client handling
2. ✅ **Structured Details**: Actionable information in error responses
3. ✅ **Type Safety**: Helper functions prevent error code typos
4. ✅ **Testability**: Integration tests verify spec accuracy
5. ✅ **Documentation Accuracy**: Runtime matches OpenAPI examples
6. ✅ **Professional Quality**: Error handling that enterprise apps expect

---

## 🔗 **What's Next?** 🚀

**Day 14 (Tomorrow)**: 
- Deploy to production environment
- Add monitoring and observability
- Rate limiting implementation
- API key authentication
- Final security hardening

**You're now ready for**: Production deployment with professional-grade error handling! 🎉

---

```
╔════════════════════════════════════════════════════════════╗
║              ACHIEVEMENT UNLOCKED                          ║
║                                                            ║
║           🎯 ERROR HANDLING GRANDMASTER 🎯                ║
║                                                            ║
║  Your API now speaks clearly when things go wrong!         ║
║                                                            ║
║  Rewards:                                                  ║
║  • Semantic error codes users can handle                   ║
║  • Structured details for debugging                        ║
║  • Tests that verify documentation accuracy                ║
║  • Runtime that matches promises                           ║
║  • Professional-grade error responses                      ║
╚════════════════════════════════════════════════════════════╝
```

**Day 13 Status**: ✅ Structured Errors Implemented | ✅ Integration Tests Passing | ✅ Spec Validated | 🎉 Production Ready!
