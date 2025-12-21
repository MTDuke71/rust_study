# Step 8: REST API with OpenAPI/Swagger - Complete Tutorial

**Learning Objective**: Build a production-ready REST API that exposes Union-Find operations with comprehensive OpenAPI documentation and interactive Swagger UI.

**Prerequisites**: 
- Completion of Steps 1-7 (understanding Union-Find implementation)
- Basic understanding of HTTP and RESTful API design
- Familiarity with async/await in Rust

**Next Steps**: After this tutorial, you'll be able to build production REST APIs for any Rust data structure or algorithm.

---

## 🎯 **What You'll Learn**

### Core Concepts
1. **Axum Web Framework** - Modern, type-safe async web framework (Backend)
2. **Utoipa (OpenAPI/Swagger)** - Compile-time API documentation generation using procedural macros
3. **State Management** - Thread-safe shared state with Arc<Mutex<T>>
4. **Error Handling** - HTTP status codes and structured error responses
5. **REST Principles** - Resource-based API design

### Production Features
- Automatic API documentation
- Interactive API testing (Swagger UI)
- Input validation
- Proper error responses
- Instance lifecycle management

---

## 🏗️ **Project Structure**

```
step8_rest_api/
├── src/
│   ├── main.rs          # Server setup and routing
│   ├── handlers.rs      # API endpoint implementations
│   ├── models.rs        # Request/Response structs with OpenAPI schemas
│   ├── state.rs         # Application state management
│   ├── openapi.rs       # OpenAPI specification
│   └── errors.rs        # Error types and responses
├── Cargo.toml           # Dependencies
├── README.md            # Quick start guide
└── TUTORIAL.md          # This file
```

---

## 📚 **Step-by-Step Learning**

### **Part 1: Understanding the Technology Stack**

#### **Axum - The Web Framework**
```rust
// Axum uses extractors for type-safe request parsing
async fn handler(
    State(app_state): State<AppState>,  // Extract application state
    Path(id): Path<Uuid>,                // Extract path parameter
    Json(payload): Json<CreateRequest>,  // Extract and parse JSON body
) -> Response {
    // Handler implementation
}
```

**Key Benefits**:
- Compile-time route checking
- Type-safe extractors
- Minimal boilerplate
- Built on Tokio for high performance

#### **Utoipa - OpenAPI Documentation**
```rust
#[derive(ToSchema)]  // Generate OpenAPI schema for this type
pub struct CreateRequest {
    #[schema(example = 10, minimum = 1)]  // Schema annotations
    pub size: usize,
}

#[utoipa::path(  // Document the endpoint
    post,
    path = "/api/v1/unionfind",
    request_body = CreateRequest,
    responses(
        (status = 201, description = "Created", body = CreateResponse)
    )
)]
async fn create_instance(...) -> Response { }
```

**Key Benefits**:
- Zero-cost abstractions (compile-time)
- No manual YAML writing
- Type-safe documentation
- Auto-generated Swagger UI

---

### **Part 2: Core Implementation Details**

#### **State Management Pattern**

**Challenge**: Union-Find needs `&mut self` for operations, but web servers handle concurrent requests.

**Solution**: Arc<Mutex<HashMap<Uuid, UnionFind>>>

```rust
#[derive(Clone)]
pub struct AppState {
    // Arc: Shared ownership across threads
    // Mutex: Exclusive access for mutations
    // HashMap: Multiple Union-Find instances by UUID
    instances: Arc<Mutex<HashMap<Uuid, UnionFind>>>,
}

impl AppState {
    // Closure pattern for safe access
    pub fn get_instance<F, R>(&self, id: Uuid, f: F) -> Option<R>
    where
        F: FnOnce(&mut UnionFind) -> R,
    {
        let mut instances = self.instances.lock().unwrap();
        instances.get_mut(&id).map(f)
    }
}
```

**Learning Points**:
- `Arc` enables shared ownership (cloneable references)
- `Mutex` provides interior mutability (exclusive access)
- Closure pattern encapsulates lock lifetime
- `Option` handles missing instances gracefully

#### **Error Handling Strategy**

**Three-Layer Error Model**:
1. **Union-Find errors** → `Result<T, String>` (e.g., out of bounds)
2. **Instance not found** → `None` from HashMap lookup
3. **HTTP errors** → StatusCode + ErrorResponse JSON

```rust
match state.get_instance(id, |uf| uf.find(element)) {
    Some(Ok(root)) => {
        // Success: 200 OK with JSON response
        (StatusCode::OK, Json(FindResponse { element, root })).into_response()
    }
    Some(Err(e)) => {
        // Union-Find error: 400 Bad Request
        error_response(StatusCode::BAD_REQUEST, &e)
    }
    None => {
        // Instance not found: 404 Not Found
        error_response(StatusCode::NOT_FOUND, "Instance not found")
    }
}
```

---

### **Part 3: API Endpoint Design**

#### **Complete API Reference**

| Method | Endpoint | Purpose | Key Learning |
|--------|----------|---------|--------------|
| POST | `/api/v1/unionfind` | Create instance | Input validation, resource creation |
| POST | `/api/v1/unionfind/{id}/union` | Union operation | Nested results, chained operations |
| GET | `/api/v1/unionfind/{id}/find` | Find root | Query parameters, idempotent operations |
| GET | `/api/v1/unionfind/{id}/connected` | Check connectivity | Multiple query params |
| GET | `/api/v1/unionfind/{id}/stats` | Get statistics | Read-only operations |
| DELETE | `/api/v1/unionfind/{id}` | Delete instance | Resource cleanup, 204 response |

#### **Endpoint Deep Dive: Union Operation**

**Challenge**: Union returns `Result<bool>`, but we also need the root after union.

```rust
pub async fn union_elements(...) -> Response {
    let result = state.get_instance(id, |uf| {
        // Chain operations: union, then find
        uf.union(element1, element2)
            .and_then(|merged| {
                // After union, get the actual root
                uf.find(element1).map(|root| (merged, root))
            })
    });
    
    // Handle nested Result<Option<Result<(bool, usize), String>>>
    match result {
        Some(Ok((merged, root))) => { /* Success */ }
        Some(Err(e)) => { /* Union-Find error */ }
        None => { /* Instance not found */ }
    }
}
```

**Learning Points**:
- `and_then` chains fallible operations
- Tuple return `(merged, root)` combines results
- Pattern matching on nested `Option<Result<T>>`
- After union, elements share the same root

---

## 🚀 **Running and Testing**

### **Start the Server**
```bash
cargo run
```

**Expected Output**:
```
🚀 Starting Union-Find REST API...
✅ Server listening on http://127.0.0.1:8080
📚 Swagger UI: http://127.0.0.1:8080/swagger-ui
```

### **Interactive Testing with Swagger UI**

1. Open http://localhost:8080/swagger-ui
2. Expand any endpoint
3. Click "Try it out"
4. Fill in parameters
5. Click "Execute"
6. View response

---

## 🧪 **Comprehensive Testing Examples**

### **Test 1: Create and Basic Operations**

```bash
# 1. Create a Union-Find instance with 10 elements
curl -X POST http://localhost:8080/api/v1/unionfind \
  -H "Content-Type: application/json" \
  -d '{"size": 10}'

# Response:
# {
#   "id": "550e8400-e29b-41d4-a716-446655440000",
#   "size": 10
# }

# Save the ID for subsequent requests
ID="550e8400-e29b-41d4-a716-446655440000"

# 2. Union elements 3 and 7
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
  -H "Content-Type: application/json" \
  -d '{"element1": 3, "element2": 7}'

# Response:
# {
#   "merged": true,
#   "root": 3
# }

# 3. Find root of element 7 (should be same as 3)
curl "http://localhost:8080/api/v1/unionfind/$ID/find?element=7"

# Response:
# {
#   "element": 7,
#   "root": 3
# }

# 4. Check if 3 and 7 are connected
curl "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=7"

# Response:
# {
#   "connected": true
# }

# 5. Check if 3 and 5 are connected (should be false)
curl "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=5"

# Response:
# {
#   "connected": false
# }
```

### **Test 2: Build Connected Components**

```bash
# Create instance
ID=$(curl -s -X POST http://localhost:8080/api/v1/unionfind \
  -H "Content-Type: application/json" \
  -d '{"size": 10}' | jq -r '.id')

# Build component: {0, 1, 2}
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
  -H "Content-Type: application/json" \
  -d '{"element1": 0, "element2": 1}'
  
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
  -H "Content-Type: application/json" \
  -d '{"element1": 1, "element2": 2}'

# Build component: {5, 6, 7}
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
  -H "Content-Type: application/json" \
  -d '{"element1": 5, "element2": 6}'
  
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
  -H "Content-Type: application/json" \
  -d '{"element1": 6, "element2": 7}'

# Check stats
curl "http://localhost:8080/api/v1/unionfind/$ID/stats"

# Response:
# {
#   "total_elements": 10,
#   "num_components": 7  # {0,1,2}, {3}, {4}, {5,6,7}, {8}, {9}
# }
```

### **Test 3: Error Handling**

```bash
# Test out-of-bounds find
curl "http://localhost:8080/api/v1/unionfind/$ID/find?element=999"

# Response (400 Bad Request):
# {
#   "code": "400 Bad Request",
#   "message": "Index 999 out of bounds for size 10"
# }

# Test non-existent instance
curl "http://localhost:8080/api/v1/unionfind/00000000-0000-0000-0000-000000000000/stats"

# Response (404 Not Found):
# {
#   "code": "404 Not Found",
#   "message": "Instance not found"
# }

# Test invalid size
curl -X POST http://localhost:8080/api/v1/unionfind \
  -H "Content-Type: application/json" \
  -d '{"size": 0}'

# Response (400 Bad Request):
# {
#   "code": "400 Bad Request",
#   "message": "Size must be at least 1"
# }
```

### **Test 4: Instance Lifecycle**

```bash
# Create instance
ID=$(curl -s -X POST http://localhost:8080/api/v1/unionfind \
  -H "Content-Type: application/json" \
  -d '{"size": 5}' | jq -r '.id')

echo "Created instance: $ID"

# Use it
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
  -H "Content-Type: application/json" \
  -d '{"element1": 0, "element2": 1}'

# Delete it
curl -X DELETE "http://localhost:8080/api/v1/unionfind/$ID"

# Response: 204 No Content (empty body)

# Try to use deleted instance
curl "http://localhost:8080/api/v1/unionfind/$ID/stats"

# Response (404 Not Found):
# {
#   "code": "404 Not Found",
#   "message": "Instance not found"
# }
```

---

## 🎓 **Understanding Exercises**

### **Exercise 1: Trace a Request**

**Question**: What happens when you POST to `/api/v1/unionfind/{id}/union`?

**Answer** (trace through the code):
1. Axum router matches route and calls `union_elements` handler
2. Extractors parse: `State(app_state)`, `Path(id)`, `Json(payload)`
3. Handler calls `state.get_instance(id, |uf| ...)`
4. State management acquires mutex lock
5. HashMap lookup finds UnionFind instance
6. Closure executes: `uf.union(e1, e2).and_then(|m| uf.find(e1).map(|r| (m, r)))`
7. Union operation connects elements (with path compression and union by rank)
8. Find operation retrieves root (with path compression)
9. Mutex released automatically (RAII)
10. Handler returns JSON response with merged flag and root

### **Exercise 2: Predict Behavior**

**Scenario**:
```bash
# Setup
curl -X POST .../unionfind -d '{"size": 5}'  # Creates ID=abc
curl -X POST .../unionfind/abc/union -d '{"element1": 0, "element2": 1}'
curl -X POST .../unionfind/abc/union -d '{"element1": 1, "element2": 2}'
curl -X POST .../unionfind/abc/union -d '{"element1": 3, "element2": 4}'

# Questions:
# 1. How many components? Answer: 2 (one containing {0,1,2}, one containing {3,4})
# 2. What's find(2)? Answer: 0 (root of component {0,1,2})
# 3. Are 2 and 4 connected? Answer: false
# 4. What happens if we union(2, 3)? Answer: merged=true, all elements now in one component
```

### **Exercise 3: Design Exercise**

**Task**: Add a new endpoint `GET /api/v1/unionfind/{id}/components` that returns all components.

**Hint**: Use `uf.components()` iterator from Mission 10.

**Expected Response**:
```json
{
  "components": [
    [0, 1, 2],
    [3, 4],
    [5],
    [6, 7, 8, 9]
  ]
}
```

---

## 🔍 **Common Mistakes and Solutions**

### **Mistake 1: Forgetting to Handle Errors**

❌ **Wrong**:
```rust
let root = state.get_instance(id, |uf| uf.find(element)).unwrap();
```

✅ **Correct**:
```rust
match state.get_instance(id, |uf| uf.find(element)) {
    Some(Ok(root)) => { /* success */ }
    Some(Err(e)) => { /* validation error */ }
    None => { /* instance not found */ }
}
```

### **Mistake 2: Hardcoding Values**

❌ **Wrong**:
```rust
Json(UnionResponse {
    merged,
    root: 0,  // Always 0!
})
```

✅ **Correct**:
```rust
uf.union(e1, e2).and_then(|merged| {
    uf.find(e1).map(|root| (merged, root))  // Actual root
})
```

### **Mistake 3: Not Validating Input**

❌ **Wrong**:
```rust
let id = state.create_instance(payload.size);  // size could be 0 or huge
```

✅ **Correct**:
```rust
if payload.size == 0 {
    return error_response(StatusCode::BAD_REQUEST, "Size must be at least 1");
}
if payload.size > 100_000 {
    return error_response(StatusCode::BAD_REQUEST, "Size exceeds maximum");
}
```

---

## 📊 **Performance Considerations**

### **Mutex Contention**

**Current**: `Arc<Mutex<HashMap<Uuid, UnionFind>>>`
- **Pros**: Simple, correct, easy to understand
- **Cons**: Single mutex for all instances (contention under heavy load)

**Production Alternative**: `Arc<DashMap<Uuid, UnionFind>>`
```rust
// DashMap: Lock-free concurrent hashmap
use dashmap::DashMap;

#[derive(Clone)]
pub struct AppState {
    instances: Arc<DashMap<Uuid, UnionFind>>,
}

// Fine-grained locking per instance
pub fn get_instance<F, R>(&self, id: Uuid, f: F) -> Option<R> {
    self.instances.get_mut(&id).map(|mut entry| f(&mut entry))
}
```

### **Benchmarking**

```bash
# Use Apache Bench for load testing
ab -n 1000 -c 10 http://localhost:8080/health

# Or wrk for more sophisticated testing
wrk -t4 -c100 -d30s --script=test.lua http://localhost:8080/api/v1/unionfind
```

---

## 🎯 **Key Takeaways**

1. ✅ **Axum uses type-safe extractors** for request parsing (State, Path, Json, Query)
2. ✅ **Utoipa generates OpenAPI docs** at compile-time from type annotations
3. ✅ **Arc<Mutex<T>> enables safe shared mutable state** across async tasks
4. ✅ **Closure pattern encapsulates lock lifetime** for clean API
5. ✅ **Three-layer error handling** maps domain errors to HTTP semantics
6. ✅ **Input validation prevents invalid operations** before they reach Union-Find
7. ✅ **DELETE returns 204 No Content** for successful deletions
8. ✅ **Swagger UI provides interactive testing** without writing test code

---

## 🚀 **Next Steps**

After mastering this tutorial, you can:
1. Add `/components` endpoint (return all disjoint sets)
2. Add `/metrics` endpoint (Prometheus-style metrics)
3. Add rate limiting with `tower-governor`
4. Add request tracing with `tracing::instrument`
5. Deploy to production (Fly.io, Railway, AWS)
6. Generate client libraries from OpenAPI spec

---

## 📚 **Further Reading**

- [Axum Documentation](https://docs.rs/axum/)
- [Utoipa Guide](https://docs.rs/utoipa/)
- [OpenAPI Specification](https://swagger.io/specification/)
- [Tokio Async Book](https://tokio.rs/tokio/tutorial)
- [Mission 10 README](../../../../../../missions/Mission10/README.md)

---

*Tutorial Status*: ✅ Complete with all endpoints, documentation, and testing examples
*Aligned with*: Mission 10 Phase 6 REST API Development
*Last Updated*: November 18, 2025
