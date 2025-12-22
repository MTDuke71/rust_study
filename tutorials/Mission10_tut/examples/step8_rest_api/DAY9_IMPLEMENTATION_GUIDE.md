# Day 9: Core REST API Implementation - Step-by-Step Guide

**Mission 10 - Day 9 Activity**  
**Goal**: Implement HTTP server with basic endpoints using Axum (REQ-8)  
**Time**: 90-120 minutes  
**Prerequisites**: Completed Day 8 (Architecture understanding)

---

## 🎯 **Today's Success Criteria**

By the end of Day 9, you should be able to:
- [ ] Understand Axum extractors (State, Path, Json, Query)
- [ ] Implement the `create_instance` handler (POST /unionfind)
- [ ] Implement the `delete_instance` handler (DELETE /unionfind/{id})
- [ ] Implement the `get_stats` handler (GET /unionfind/{id}/stats)
- [ ] Test endpoints with curl or Swagger UI
- [ ] Understand the request/response lifecycle

**You will NOT yet**: Implement union/find operations (that's Day 10) or add complete Utoipa annotations (Day 11)

---

## 🔄 **Day 8 → Day 9 Transition**

### **What You Already Know (Day 8)**
- ✅ Tech stack: Axum, Utoipa, Tokio
- ✅ File structure: `main.rs`, `handlers.rs`, `models.rs`, `state.rs`, `errors.rs`
- ✅ State management: `Arc<Mutex<HashMap<Uuid, UnionFind>>>`
- ✅ All 6 endpoints and their purposes

### **What You'll Learn Today (Day 9)**
- 🆕 How Axum extractors work (type-safe request parsing)
- 🆕 Implementing handlers step-by-step
- 🆕 Request/response patterns
- 🆕 Error handling in practice
- 🆕 Testing with real HTTP requests

---

## 📋 **Step 1: Understanding Axum Extractors (20 minutes)**

### **1.1 What Are Extractors?**

**Traditional Web** (Express.js):
```javascript
app.post('/api/v1/unionfind/:id/union', (req, res) => {
    const id = req.params.id;           // Manual extraction
    const body = req.body;              // Manual parsing
    const state = req.app.locals.state; // Manual state access
    
    // Need to validate types manually!
    if (typeof body.element1 !== 'number') {
        return res.status(400).json({ error: "Invalid input" });
    }
    // ... more validation
});
```

**Axum** (Rust):
```rust
async fn union_elements(
    State(app_state): State<AppState>,     // ✅ Auto-extracted, type-safe
    Path(id): Path<Uuid>,                  // ✅ Auto-parsed, validated
    Json(payload): Json<UnionRequest>,     // ✅ Auto-deserialized, validated
) -> Response {
    // If this function runs, ALL types are GUARANTEED valid!
    // No manual validation needed for basic types
}
```

**Key benefit**: If the types don't match, Axum automatically returns 400 Bad Request BEFORE your handler runs!

### **1.2 The Four Essential Extractors**

| **Extractor** | **Extracts From** | **Example** | **Use Case** |
|---------------|-------------------|-------------|--------------|
| `State<T>` | Application state | `State(app_state): State<AppState>` | Access shared data (Arc<Mutex<HashMap>>) |
| `Path<T>` | URL path segments | `Path(id): Path<Uuid>` | Extract `/unionfind/{id}` → `id` |
| `Json<T>` | Request body | `Json(payload): Json<CreateRequest>` | Parse JSON POST body |
| `Query<T>` | Query parameters | `Query(params): Query<FindQuery>` | Parse `?element=5` |

### **1.3 How Extractors Work (Mental Model)**

```rust
// When request arrives: POST /api/v1/unionfind/abc-123/union
// Body: {"element1": 3, "element2": 7}

async fn union_elements(
    State(app_state): State<AppState>,     // Axum clones app_state from router
    Path(id): Path<Uuid>,                  // Axum parses "abc-123" as Uuid
    Json(payload): Json<UnionRequest>,     // Axum deserializes JSON to struct
) -> Response {
    // Step 1: Axum validated id is a valid UUID (or returns 400)
    // Step 2: Axum validated JSON matches UnionRequest schema (or returns 400)
    // Step 3: Only NOW does your code run with guaranteed-valid data!
    
    println!("ID: {}", id);                    // Valid Uuid
    println!("Elements: {} and {}", 
             payload.element1,                 // Valid usize
             payload.element2);                // Valid usize
}
```

**Behind the scenes** (you don't write this, Axum does):
```rust
// Pseudo-code of what Axum does
fn handle_request(request: Request) {
    // Extract state
    let app_state = request.extensions.get::<AppState>().unwrap();
    
    // Parse path parameter
    let id = match Uuid::parse_str(path_param) {
        Ok(uuid) => uuid,
        Err(_) => return StatusCode::BAD_REQUEST,  // Auto-reject!
    };
    
    // Deserialize JSON
    let payload = match serde_json::from_slice::<UnionRequest>(body) {
        Ok(p) => p,
        Err(_) => return StatusCode::BAD_REQUEST,  // Auto-reject!
    };
    
    // Now call YOUR handler
    union_elements(app_state, id, payload).await
}
```

---

## 📋 **Step 2: Implement `create_instance` Handler (25 minutes)**

### **2.1 Review the Model**

Open `src/models.rs` and find:

```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRequest {
    #[schema(example = 10, minimum = 1)]
    pub size: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResponse {
    pub id: Uuid,
    pub size: usize,
}
```

**What this means**:
- Client sends: `{"size": 10}`
- Server returns: `{"id": "abc-123", "size": 10}`

### **2.2 Understand the Endpoint**

**Endpoint**: `POST /api/v1/unionfind`  
**Purpose**: Create a new Union-Find instance  
**Request**: JSON body with `size`  
**Response**: 201 Created with `id` and `size`

### **2.3 Implementation Walkthrough**

Open `src/handlers.rs` and find `create_instance`:

```rust
#[utoipa::path(
    post,
    path = "/api/v1/unionfind",
    request_body = CreateRequest,
    responses(
        (status = 201, description = "Union-Find instance created", body = CreateResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "union-find"
)]
pub async fn create_instance(
    State(state): State<AppState>,           // 1️⃣ Extract shared state
    Json(payload): Json<CreateRequest>,      // 2️⃣ Extract and parse JSON body
) -> Response {
    // 3️⃣ Validate input (business logic, not type validation)
    if payload.size == 0 {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Size must be at least 1",
        );
    }

    // Optional: Add maximum size limit
    if payload.size > 100_000 {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Size exceeds maximum of 100,000",
        );
    }

    // 4️⃣ Create Union-Find instance via state manager
    let id = state.create_instance(payload.size);

    // 5️⃣ Build response struct
    let response = CreateResponse {
        id,
        size: payload.size,
    };

    // 6️⃣ Return HTTP 201 Created with JSON body
    (StatusCode::CREATED, Json(response)).into_response()
}
```

### **2.4 Step-by-Step Explanation**

**1️⃣ Extract State**:
```rust
State(state): State<AppState>
```
- Axum gives you a clone of the `Arc<AppState>` from the router
- Cheap to clone (just increments reference count)
- All handlers share the SAME underlying HashMap

**2️⃣ Extract JSON Body**:
```rust
Json(payload): Json<CreateRequest>
```
- Axum reads HTTP body bytes
- Deserializes using `serde_json`
- If deserialization fails (invalid JSON or wrong shape), returns 400 automatically

**3️⃣ Validate Business Rules**:
```rust
if payload.size == 0 { /* reject */ }
```
- Type validation is automatic (Axum did this)
- Business validation is YOUR job (size > 0, size < max, etc.)

**4️⃣ Call State Method**:
```rust
let id = state.create_instance(payload.size);
```
- Locks mutex briefly
- Creates `UnionFind::new(payload.size)`
- Inserts into HashMap with generated UUID
- Returns the UUID

**5️⃣ Build Response**:
```rust
let response = CreateResponse { id, size: payload.size };
```
- Create the response struct
- Will be serialized to JSON automatically

**6️⃣ Return HTTP Response**:
```rust
(StatusCode::CREATED, Json(response)).into_response()
```
- HTTP 201 Created (resource created)
- JSON body with `id` and `size`
- `.into_response()` converts to Axum's Response type

### **2.5 Activity: Test with curl**

```powershell
# Start server (if not running)
cd tutorials/Mission10_tut/examples/step8_rest_api
cargo run

# In another terminal, create an instance
curl -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 10}'

# Expected output:
# {"id":"550e8400-e29b-41d4-a716-446655440000","size":10}
```

**Save the ID** for next tests:
```powershell
# PowerShell syntax
$UF_ID = "550e8400-e29b-41d4-a716-446655440000"
```

---

## 📋 **Step 3: Implement `get_stats` Handler (20 minutes)**

### **3.1 Review the Model**

```rust
// No request body needed (GET request)

#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    pub size: usize,
    pub num_components: usize,
}
```

### **3.2 Understand the Endpoint**

**Endpoint**: `GET /api/v1/unionfind/{id}/stats`  
**Purpose**: Get statistics about an existing instance  
**Request**: No body, just path parameter `{id}`  
**Response**: 200 OK with `size` and `num_components`

### **3.3 Implementation Walkthrough**

```rust
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/stats",
    params(
        ("id" = Uuid, Path, description = "Union-Find instance ID")
    ),
    responses(
        (status = 200, description = "Statistics retrieved", body = StatsResponse),
        (status = 404, description = "Instance not found", body = ErrorResponse)
    ),
    tag = "union-find"
)]
pub async fn get_stats(
    State(state): State<AppState>,     // 1️⃣ Extract state
    Path(id): Path<Uuid>,              // 2️⃣ Extract UUID from path
) -> Response {
    // 3️⃣ Try to access instance and get stats
    match state.get_instance(id, |uf| (uf.len(), uf.num_components())) {
        Some((size, num_components)) => {
            // 4️⃣ Success: instance exists
            let response = StatsResponse {
                size,
                num_components,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        None => {
            // 5️⃣ Failure: instance not found
            error_response(
                StatusCode::NOT_FOUND,
                &format!("Union-Find instance {} not found", id),
            )
        }
    }
}
```

### **3.4 Step-by-Step Explanation**

**1️⃣ Extract State** (same as before)

**2️⃣ Extract Path Parameter**:
```rust
Path(id): Path<Uuid>
```
- From URL `/api/v1/unionfind/abc-123/stats`
- Axum extracts `"abc-123"` and parses as Uuid
- If invalid UUID format, Axum returns 400 automatically

**3️⃣ Access Instance with Closure**:
```rust
state.get_instance(id, |uf| (uf.len(), uf.num_components()))
```
- `state.get_instance()` locks mutex, looks up `id` in HashMap
- If found: calls closure with `&mut UnionFind` reference
- Closure returns `(size, num_components)` tuple
- Mutex unlocked automatically after closure returns
- Returns `Option<(usize, usize)>`

**4️⃣ Success Case**:
```rust
Some((size, num_components)) => {
    let response = StatsResponse { size, num_components };
    (StatusCode::OK, Json(response)).into_response()
}
```
- Instance exists → HTTP 200 OK
- Return stats as JSON

**5️⃣ Failure Case**:
```rust
None => {
    error_response(StatusCode::NOT_FOUND, &format!("...not found"))
}
```
- Instance doesn't exist → HTTP 404 Not Found
- Return error JSON: `{"code": 404, "message": "...not found"}`

### **3.5 Activity: Test with curl**

```powershell
# Get stats for the instance you created earlier
curl http://localhost:8080/api/v1/unionfind/$UF_ID/stats

# Expected output (initially all elements are separate):
# {"size":10,"num_components":10}PS D:\...>
# Note: Prompt appears on same line - that's normal! JSON has no trailing newline.

# Curl also shows progress meter (% Total, % Received, etc.)
# To hide progress meter and get clean output, use -s (silent) flag:
curl -s http://localhost:8080/api/v1/unionfind/$UF_ID/stats

# For prettier output with proper newline:
curl -s http://localhost:8080/api/v1/unionfind/$UF_ID/stats | ConvertFrom-Json | ConvertTo-Json

# Test 404 error with invalid ID
curl http://localhost:8080/api/v1/unionfind/00000000-0000-0000-0000-000000000000/stats

# Expected output:
# {"code":404,"message":"Union-Find instance 00000000-0000-0000-0000-000000000000 not found"}
```

---

## 📋 **Step 4: Implement `delete_instance` Handler (15 minutes)**

### **4.1 Understand the Endpoint**

**Endpoint**: `DELETE /api/v1/unionfind/{id}`  
**Purpose**: Delete an existing instance  
**Request**: No body, just path parameter `{id}`  
**Response**: 204 No Content (success) or 404 Not Found

### **4.2 Implementation Walkthrough**

```rust
#[utoipa::path(
    delete,
    path = "/api/v1/unionfind/{id}",
    params(
        ("id" = Uuid, Path, description = "Union-Find instance ID")
    ),
    responses(
        (status = 204, description = "Instance deleted successfully"),
        (status = 404, description = "Instance not found", body = ErrorResponse)
    ),
    tag = "union-find"
)]
pub async fn delete_instance(
    State(state): State<AppState>,     // 1️⃣ Extract state
    Path(id): Path<Uuid>,              // 2️⃣ Extract UUID
) -> Response {
    // 3️⃣ Try to delete instance
    match state.delete_instance(id) {
        true => {
            // 4️⃣ Success: instance existed and was deleted
            StatusCode::NO_CONTENT.into_response()
        }
        false => {
            // 5️⃣ Failure: instance not found
            error_response(
                StatusCode::NOT_FOUND,
                &format!("Union-Find instance {} not found", id),
            )
        }
    }
}
```

### **4.3 Step-by-Step Explanation**

**3️⃣ Delete Instance**:
```rust
state.delete_instance(id)
```
- Locks mutex
- Calls `HashMap::remove(id)`
- Returns `true` if key existed, `false` otherwise

**4️⃣ Success Case** (HTTP 204):
```rust
StatusCode::NO_CONTENT.into_response()
```
- **Important**: DELETE success returns **204 No Content**, NOT 200 OK
- No response body (the resource no longer exists)
- This is standard RESTful practice

**5️⃣ Failure Case** (HTTP 404):
```rust
error_response(StatusCode::NOT_FOUND, ...)
```
- Instance doesn't exist → 404 Not Found
- Return error JSON

### **4.4 Activity: Test with curl**

```powershell
# Delete the instance
curl -X DELETE http://localhost:8080/api/v1/unionfind/$UF_ID

# Expected: No output (HTTP 204 No Content)

# Try to get stats again (should fail)
curl http://localhost:8080/api/v1/unionfind/$UF_ID/stats

# Expected output:
# {"code":404,"message":"Union-Find instance ... not found"}

# Try to delete again (should also fail)
curl -X DELETE http://localhost:8080/api/v1/unionfind/$UF_ID

# Expected output:
# {"code":404,"message":"Union-Find instance ... not found"}
```

---

## 📋 **Step 5: Understanding Error Handling (15 minutes)**

### **5.1 The Error Helper Function**

Open `src/handlers.rs` and find the `error_response` helper function:

```rust
fn error_response(code: StatusCode, message: &str) -> Response {
    (
        code,
        Json(ErrorResponse {
            code: code.to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}
```

**What it does**:
- Takes an HTTP status code and error message
- Creates an `ErrorResponse` JSON structure
- Returns a properly formatted HTTP error response

### **5.2 Error Response Pattern**

**Usage in handlers**:
```rust
// 400 Bad Request (client error)
return error_response(
    StatusCode::BAD_REQUEST,
    "Size must be at least 1",
);

// 404 Not Found (resource doesn't exist)
return error_response(
    StatusCode::NOT_FOUND,
    &format!("Union-Find instance {} not found", id),
);

// 500 Internal Server Error (server error)
return error_response(
    StatusCode::INTERNAL_SERVER_ERROR,
    "Unexpected error occurred",
);
```

### **5.3 HTTP Status Code Guidelines**

| **Code** | **Name** | **Use When** | **Example** |
|----------|----------|--------------|-------------|
| **200** | OK | Successful GET/POST operation | Returning stats or union result |
| **201** | Created | Resource created successfully | Creating new Union-Find instance |
| **204** | No Content | Successful DELETE with no response | Deleting an instance |
| **400** | Bad Request | Invalid input from client | size=0, invalid JSON format |
| **404** | Not Found | Resource doesn't exist | UUID not in HashMap |
| **500** | Internal Server Error | Unexpected server error | Panic, database failure, etc. |

### **5.4 Activity: Trigger Each Error Type**

```powershell
# Create instance for testing
$response = curl -s -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 10}' | ConvertFrom-Json
$ID = $response.id

# Test 400 Bad Request (invalid size)
curl -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 0}'

# Output: {"code":400,"message":"Size must be at least 1"}

# Test 400 Bad Request (invalid JSON)
curl -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{invalid json}'

# Output: Axum's automatic JSON parsing error (HTTP 400)
# Failed to parse the request body as JSON: key must be a string at line 1 column 2

# Test 404 Not Found
curl http://localhost:8080/api/v1/unionfind/00000000-0000-0000-0000-000000000000/stats

# Output: {"code":404,"message":"Union-Find instance ... not found"}

# Test 200 OK
curl http://localhost:8080/api/v1/unionfind/$ID/stats

# Output: {"total_elements":10,"num_components":10}
```

---

## 📋 **Step 6: Complete Request/Response Lifecycle (15 minutes)**

### **6.1 Trace a Complete Request**

Let's trace: `POST /api/v1/unionfind` with `{"size": 10}`

```
1. Client sends HTTP POST
   ↓
2. Axum receives request, parses HTTP headers
   ↓
3. Axum matches route: POST /api/v1/unionfind → create_instance handler
   ↓
4. Axum extracts State<AppState> from router extensions
   ↓
5. Axum reads request body bytes
   ↓
6. Axum deserializes JSON to CreateRequest struct
   ├─ Success → Continue to step 7
   └─ Failure → Return 400 Bad Request (Axum handles this)
   ↓
7. Axum calls create_instance(state, payload)
   ↓
8. Handler validates payload.size > 0
   ├─ Valid → Continue
   └─ Invalid → Return error_response(400, "Size must be at least 1")
   ↓
9. Handler calls state.create_instance(payload.size)
   ├─ Locks mutex
   ├─ Creates UnionFind::new(10)
   ├─ Generates UUID
   ├─ Inserts into HashMap
   └─ Releases mutex
   ↓
10. Handler builds CreateResponse { id, size }
   ↓
11. Handler returns (StatusCode::CREATED, Json(response))
   ↓
12. Axum serializes CreateResponse to JSON bytes
   ↓
13. Axum sets HTTP status 201 and Content-Type: application/json header
   ↓
14. Axum sends HTTP response to client
```

### **6.2 What Happens Under the Hood (State Access)**

```rust
// When you call:
state.get_instance(id, |uf| uf.num_components())

// This happens:
impl AppState {
    pub fn get_instance<F, R>(&self, id: Uuid, f: F) -> Option<R>
    where
        F: FnOnce(&mut UnionFind) -> R,
    {
        // 1. Lock mutex (blocks if another thread holds lock)
        let mut instances = self.instances.lock().unwrap();
        
        // 2. Look up UUID in HashMap
        instances.get_mut(&id).map(|uf| {
            // 3. If found, call closure with mutable reference
            f(uf)
            // 4. Mutex unlocked automatically here (RAII)
        })
        // 5. Return Option<R> (Some if found, None if not)
    }
}
```

**Key points**:
- ✅ Mutex ensures only ONE handler accesses data at a time
- ✅ Lock is held ONLY during closure execution (very brief)
- ✅ Lock released automatically (RAII - no manual unlock needed)
- ✅ `Option` pattern handles "instance not found" gracefully

---

## 📋 **Step 7: Testing with Swagger UI (10 minutes)**

### **7.1 Start Server and Open Swagger**

```bash
# Ensure server is running
cargo run

# Open browser
# http://localhost:8080/swagger-ui
```

### **7.2 Test Create Instance**

1. Expand "POST /api/v1/unionfind"
2. Click "Try it out"
3. Modify size if desired: `{"size": 20}`
4. Click "Execute"
5. **Copy the `id` from the response** (you'll need it!)

**Expected Response** (HTTP 201):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "size": 20
}
```

### **7.3 Test Get Stats**

1. Expand "GET /api/v1/unionfind/{id}/stats"
2. Click "Try it out"
3. **Paste the ID** you copied earlier
4. Click "Execute"

**Expected Response** (HTTP 200):
```json
{
  "total_elements": 20,
  "num_components": 20
}
```

### **7.4 Test Delete Instance**

1. Expand "DELETE /api/v1/unionfind/{id}"
2. Click "Try it out"
3. **Paste the same ID**
4. Click "Execute"

**Expected Response**: HTTP 204 No Content (no body)

### **7.5 Test 404 Error**

1. Try to get stats again with the same ID
2. **Expected**: HTTP 404 with error message

---

## 📋 **Step 8: Document Your Understanding (10 minutes)**

### **8.1 Create Day 9 Notes**

Create `DAY9_NOTES.md`:

```markdown
# Day 9: Core REST API Implementation Notes

## Axum Extractors Implemented

### State<AppState>
- **Purpose**: Access shared application state
- **Pattern**: `State(state): State<AppState>`
- **Usage**: Every handler needs this to access HashMap of instances

### Path<Uuid>
- **Purpose**: Extract UUID from URL path
- **Pattern**: `Path(id): Path<Uuid>`
- **Auto-validation**: Axum returns 400 if not valid UUID format

### Json<T>
- **Purpose**: Parse JSON request body
- **Pattern**: `Json(payload): Json<CreateRequest>`
- **Auto-validation**: Axum returns 400 if JSON doesn't match struct

## Handlers Implemented

### 1. create_instance (POST /api/v1/unionfind)
- **Input**: CreateRequest (size)
- **Output**: 201 Created + CreateResponse (id, size)
- **Validation**: size > 0 and size <= 100,000
- **State mutation**: Adds new UUID → UnionFind to HashMap

### 2. get_stats (GET /api/v1/unionfind/{id}/stats)
- **Input**: UUID path parameter
- **Output**: 200 OK + StatsResponse (size, num_components)
- **Error**: 404 if instance not found
- **State access**: Read-only via closure

### 3. delete_instance (DELETE /api/v1/unionfind/{id})
- **Input**: UUID path parameter
- **Output**: 204 No Content (success) or 404 Not Found
- **State mutation**: Removes UUID from HashMap
- **RESTful pattern**: 204 for successful deletion with no body

## Error Handling Patterns

| **Status** | **Use Case** | **Example** |
|------------|--------------|-------------|
| 400 | Invalid input | size=0, invalid JSON |
| 404 | Instance not found | UUID doesn't exist |
| 200 | Successful read | Stats retrieved |
| 201 | Resource created | New instance created |
| 204 | Successful delete | Instance removed |

## Key Learnings

1. **Type safety**: Extractors validate types before handler runs
2. **Option pattern**: `state.get_instance()` returns `Option<R>` for not-found cases
3. **Closure pattern**: Encapsulates mutex lifetime for clean API
4. **RESTful semantics**: Different status codes for different operations
5. **Auto-serialization**: `Json(response)` automatically converts struct to JSON

## Testing Workflow

```powershell
# Create instance
$response = curl -s -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 10}' | ConvertFrom-Json
$ID = $response.id

# Get stats
curl http://localhost:8080/api/v1/unionfind/$ID/stats

# Delete instance
curl -X DELETE http://localhost:8080/api/v1/unionfind/$ID
```

## Tomorrow (Day 10)

- Implement union_elements (POST /unionfind/{id}/union)
- Implement find_root (GET /unionfind/{id}/find)
- Implement check_connected (GET /unionfind/{id}/connected)
- Handle Union-Find operation errors
```

---

## ✅ **Day 9 Completion Checklist**

By now you should have:

- [x] **Understood extractors**: State, Path, Json, and their purposes
- [x] **Implemented create_instance**: POST handler with validation
- [x] **Implemented get_stats**: GET handler with path parameter
- [x] **Implemented delete_instance**: DELETE handler with 204 response
- [x] **Tested with curl**: All three endpoints working
- [x] **Tested with Swagger**: Interactive API testing
- [x] **Created notes**: DAY9_NOTES.md documenting handlers
- [x] **Traced request lifecycle**: Understanding the complete flow

## 🚫 **What You Should NOT Know Yet**

It's OK if you don't understand:
- ❌ Union/find operation handlers (Day 10)
- ❌ Query extractor for `?element=5` (Day 10)
- ❌ Handling Union-Find errors in handlers (Day 10)
- ❌ Complete Utoipa annotations (Day 11)
- ❌ Advanced async patterns (covered in Rustaceans)

---

## 🎯 **Day 10 Preview**

Tomorrow you'll:
1. Implement `union_elements` (POST /unionfind/{id}/union)
2. Implement `find_root` (GET /unionfind/{id}/find?element=5)
3. Implement `check_connected` (GET /unionfind/{id}/connected?element1=3&element2=7)
4. Use `Query<T>` extractor for query parameters
5. Handle three-layer error model (Union-Find errors vs. HTTP errors)
6. Complete the full API implementation

**For now**: REST! You've completed Day 9's core implementation phase. Tomorrow you finish the remaining endpoints!

---

## 📊 **Quick Reference**

### **Handler Template**
```rust
#[utoipa::path(/* ... */)]
pub async fn handler_name(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<RequestType>,
) -> Response {
    // 1. Validate input (business rules)
    if invalid { return error_response(...); }
    
    // 2. Access state with closure
    match state.operation(|uf| /* ... */) {
        Some(result) => {
            // 3. Build response
            let response = ResponseType { /* ... */ };
            (StatusCode::OK, Json(response)).into_response()
        }
        None => {
            // 4. Handle not found
            error_response(StatusCode::NOT_FOUND, "...")
        }
    }
}
```

### **curl Testing Template**
```powershell
# POST with JSON body
curl -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 10}'

# GET with path parameter
curl http://localhost:8080/api/v1/unionfind/$UF_ID/stats

# DELETE with path parameter
curl -X DELETE http://localhost:8080/api/v1/unionfind/$UF_ID

# PowerShell tip: Pretty print JSON output
curl http://localhost:8080/api/v1/unionfind/$UF_ID/stats | ConvertFrom-Json | ConvertTo-Json
```

---

**Day 9 Status**: ✅ Core REST API Implementation Complete  
**Next**: Day 10 - Complete All Endpoints (Union/Find Operations)  
**Reference**: [TUTORIAL.md](TUTORIAL.md) sections "Part 2: Core Implementation Details"
