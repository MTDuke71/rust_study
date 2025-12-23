# Day 10: Complete Union/Find Operation Endpoints - Step-by-Step Guide

**Mission 10 - Day 10 Activity**  
**Goal**: Implement remaining Union/Find operation endpoints (REQ-8 completion)  
**Time**: 90-120 minutes  
**Prerequisites**: Completed Day 9 (Core endpoints: create, delete, stats)

---

## 🎯 **Today's Success Criteria**

By the end of Day 10, you should be able to:
- [ ] Understand the `Query<T>` extractor for query parameters
- [ ] Implement the `union_elements` handler (POST /unionfind/{id}/union)
- [ ] Implement the `find_element` handler (GET /unionfind/{id}/find)
- [ ] Implement the `check_connected` handler (GET /unionfind/{id}/connected)
- [ ] Handle Union-Find operation errors (element out of bounds)
- [ ] Test all six endpoints together with curl/Swagger
- [ ] Understand the complete API request flow

**You will NOT yet**: Add complete Utoipa documentation annotations (that's Day 11)

---

## 🔄 **Day 9 → Day 10 Transition**

### **What You Already Know (Day 9)**
- ✅ Extractors: `State<AppState>`, `Path<Uuid>`, `Json<T>`
- ✅ Handlers: `create_instance`, `delete_instance`, `get_stats`
- ✅ Error handling: `error_response` helper, HTTP status codes
- ✅ Request lifecycle: extractors → validation → state access → response
- ✅ Testing: curl commands and Swagger UI

### **What You'll Learn Today (Day 10)**
- 🆕 `Query<T>` extractor for URL query parameters (`?element=5`)
- 🆕 Implementing Union-Find operation handlers
- 🆕 Three-layer error handling (type errors → business errors → HTTP errors)
- 🆕 Different request patterns (POST with body vs. GET with query params)
- 🆕 Complete API integration testing

---

## 📋 **Step 1: Understanding Query Parameters (20 minutes)**

### **1.1 Query Parameters vs. Path Parameters**

| **Type** | **Example URL** | **Use Case** | **Extractor** |
|----------|-----------------|--------------|---------------|
| **Path Parameter** | `/unionfind/{id}/stats` | Resource identification | `Path<Uuid>` |
| **Query Parameter** | `/unionfind/{id}/find?element=5` | Optional filters/parameters | `Query<T>` |

**Path parameters** (Day 9):
- Part of the URL structure
- Usually required
- Identify a specific resource
- Example: `/unionfind/abc-123/stats` → `id = "abc-123"`

**Query parameters** (Today):
- After the `?` in URL
- Often optional (filtering, pagination)
- Provide additional operation parameters
- Example: `/unionfind/abc-123/find?element=5` → `element = 5`

### **1.2 The `Query<T>` Extractor**

**Traditional Web** (Express.js):
```javascript
app.get('/api/v1/unionfind/:id/find', (req, res) => {
    const id = req.params.id;
    const element = req.query.element;  // Manual extraction
    
    // Need to validate types manually!
    if (typeof element !== 'number') {
        return res.status(400).json({ error: "Invalid element" });
    }
    // ... more validation
});
```

**Axum** (Rust):
```rust
#[derive(Debug, Deserialize)]
pub struct FindRequest {
    pub element: usize,
}

async fn find_element(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<FindRequest>,    // ✅ Auto-extracted and validated!
) -> Response {
    // If this runs, query.element is GUARANTEED to be a valid usize!
    println!("Finding element {}", query.element);
}
```

**Key benefit**: Just like `Json<T>`, if the query parameter doesn't parse to the correct type, Axum returns 400 automatically!

### **1.3 Query Parameter Examples**

```rust
// Single parameter
#[derive(Debug, Deserialize, ToSchema)]
pub struct FindRequest {
    #[schema(example = 5, minimum = 0)]
    pub element: usize,
}
// URL: /find?element=5

// Multiple parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectedRequest {
    #[schema(example = 3, minimum = 0)]
    pub element1: usize,
    #[schema(example = 7, minimum = 0)]
    pub element2: usize,
}
// URL: /connected?element1=3&element2=7

// Optional parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub element: usize,
    pub verbose: Option<bool>,  // Optional: /find?element=5&verbose=true
}
```

### **1.4 How Query Extraction Works**

```rust
// When request arrives: GET /api/v1/unionfind/abc-123/find?element=5

async fn find_element(
    State(state): State<AppState>,     // Axum clones app_state
    Path(id): Path<Uuid>,              // Axum parses "abc-123" as UUID
    Query(params): Query<FindRequest>,    // Axum parses "element=5" to struct
) -> Response {
    // Step 1: Axum validated id is valid UUID (or returns 400)
    // Step 2: Axum parsed query string to FindRequest struct
    //         - Found "element=5"
    //         - Parsed "5" as usize
    //         - If parsing failed or parameter missing, returns 400
    // Step 3: Your code runs with guaranteed-valid data!
    
    println!("ID: {}", id);            // Valid UUID
    println!("Element: {}", params.element);  // Valid usize
}
```

**Behind the scenes** (Axum does this):
```rust
// Pseudo-code
fn handle_request(request: Request) {
    // Parse query string "element=5"
    let params = match serde_urlencoded::from_str::<FindRequest>("element=5") {
        Ok(q) => q,
        Err(_) => return StatusCode::BAD_REQUEST,  // Auto-reject!
    };
    
    // Now call YOUR handler
    find_element(app_state, id, params).await
}
```

---

## 📋 **Step 2: Implement `find_element` Handler (25 minutes)**

### **2.1 Review the Models**

Open `src/models.rs` and find:

```rust
// Query parameter struct
#[derive(Debug, Deserialize, ToSchema)]
pub struct FindRequest {
    /// Element to find root for
    pub element: usize,
}

// Response struct
#[derive(Debug, Serialize, ToSchema)]
pub struct FindResponse {
    /// The element queried
    pub element: usize,
    /// The representative (root) of the set
    pub root: usize,
}
```

**What this means**:
- Client sends: `GET /unionfind/abc-123/find?element=5`
- Server returns: `{"element": 5, "root": 2}` (if 5's root is 2)

### **2.2 Understand the Endpoint**

**Endpoint**: `GET /api/v1/unionfind/{id}/find?element=N`  
**Purpose**: Find the root of a given element  
**Request**: Path parameter `{id}`, query parameter `element`  
**Response**: 200 OK with `element` and `root`  
**Errors**: 
- 404 if instance not found
- 400 if element is out of bounds

### **2.3 Implementation Walkthrough**

Open `src/handlers.rs` and find `find_element`:

```rust
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/find",
    params(
        ("id" = Uuid, Path, description = "Instance ID"),
        ("element" = usize, Query, description = "Element to find root for")
    ),
    responses(
        (status = 200, description = "Find operation successful", body = FindResponse),
        (status = 404, description = "Instance not found"),
        (status = 400, description = "Invalid element index")
    ),
    tag = "Operations"
)]
pub async fn find_element(
    State(state): State<AppState>,         // 1️⃣ Extract state
    Path(id): Path<Uuid>,                  // 2️⃣ Extract UUID from path
    Query(params): Query<FindRequest>,     // 3️⃣ Extract query parameters
) -> Response {
    // 4️⃣ Access instance and perform find operation
    let result = state.get_instance(id, |uf| {
        uf.find(params.element)
    });

    match result {
        Some(Ok(root)) => (
            // 5️⃣ Success: element valid, root found
            StatusCode::OK,
            Json(FindResponse {
                element: params.element,
                root,
            }),
        ).into_response(),
        Some(Err(e)) => {
            // 6️⃣ Element out of bounds error
            error_response(StatusCode::BAD_REQUEST, &e)
        }
        None => {
            // 7️⃣ Instance not found
            error_response(StatusCode::NOT_FOUND, "Instance not found")
        }
    }
}
```

### **2.4 Step-by-Step Explanation**

**1️⃣-2️⃣ Extract State and Path** (same as Day 9)

**3️⃣ Extract Query Parameters**:
```rust
Query(params): Query<FindRequest>
```
- From URL `/api/v1/unionfind/abc-123/find?element=5`
- Axum parses query string `element=5`
- Deserializes to `FindRequest { element: 5 }`
- If parameter missing or invalid type, Axum returns 400 automatically

**4️⃣ Perform Find Operation**:
```rust
state.get_instance(id, |uf| uf.find(params.element))
```
- Returns `Option<Result<usize, UnionFindError>>`
- `Option`: Did we find the instance? (Some/None)
- `Result`: Did the operation succeed? (Ok/Err)
- **Three-layer error model**:
  - `None` → Instance not found (404)
  - `Some(Err(_))` → Operation failed (400)
  - `Some(Ok(root))` → Success (200)

**5️⃣ Success Case**:
```rust
Some(Ok(root)) => (
    StatusCode::OK,
    Json(FindResponse {
        element: params.element,
        root,
    }),
).into_response()
```
- Element is valid (within bounds)
- Found the root successfully
- Return HTTP 200 with element and root

**6️⃣ Element Out of Bounds**:
```rust
Some(Err(e)) => error_response(StatusCode::BAD_REQUEST, &e)
```
- Instance exists BUT element is invalid
- `uf.find()` returned `Err` with error message
- Return HTTP 400 (client provided invalid element)

**7️⃣ Instance Not Found** (same as Day 9):
```rust
None => error_response(StatusCode::NOT_FOUND, "Instance not found")
```

### **2.5 Understanding the Three-Layer Error Model**

The actual implementation uses a clean two-step pattern (more idiomatic Rust):

```rust
// Step 1: Extract result (separates operation from pattern matching)
let result = state.get_instance(id, |uf| {
    uf.find(params.element)
});

// Step 2: Match on nested Option<Result<T, E>>
match result {
    Some(Ok(root)) => (StatusCode::OK, Json(response)).into_response(),
    Some(Err(e)) => error_response(StatusCode::BAD_REQUEST, &e),
    None => error_response(StatusCode::NOT_FOUND, "Instance not found"),
}
```

**Three-layer error handling**:

**Layer 1: Type validation** (Axum automatic)
- `Query(params): Query<FindRequest>`
- If query param can't parse to usize → 400 **before handler runs**
- Your code never sees invalid types

**Layer 2: Resource existence** (AppState)
- `state.get_instance(id, |uf| ...)` returns `Option<Result<T, E>>`
- `None` = instance not found → **404**
- `Some(...)` = instance exists, check operation result

**Layer 3: Operation validation** (UnionFind)
- `uf.find(element)` returns `Result<usize, String>`
- `Err(e)` = element out of bounds → **400** (business logic error)
- `Ok(root)` = operation succeeded → **200**

**Why this pattern is idiomatic**:
- ✅ Separates operation (`let result = ...`) from pattern matching
- ✅ Single `match` handles all cases clearly
- ✅ No nested matches (cleaner code flow)
- ✅ Error messages directly from Mission 10's UnionFind

### **2.6 Activity: Test with curl**

```powershell
# Create a fresh instance
$response = curl -s -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 10}' | ConvertFrom-Json
$ID = $response.id
Write-Host "Created instance: $ID"

# Test find_element with valid element
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=5" | ConvertFrom-Json | ConvertTo-Json

# Expected output (initially, each element is its own root):
# {
#   "element": 5,
#   "root": 5
# }

# Test with element 0
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=0" | ConvertFrom-Json | ConvertTo-Json

# Expected: {"element": 0, "root": 0}

# Test element out of bounds (size is 10, so valid range is 0-9)
curl "http://localhost:8080/api/v1/unionfind/$ID/find?element=10"

# Expected: {"code":"400 Bad Request","message":"Element 10 is out of bounds"}

# Test with invalid instance ID
curl "http://localhost:8080/api/v1/unionfind/00000000-0000-0000-0000-000000000000/find?element=5"

# Expected: {"code":"404 Not Found","message":"Union-Find instance ... not found"}

# Test with invalid query parameter type (string instead of number)
curl "http://localhost:8080/api/v1/unionfind/$ID/find?element=abc"

# Expected: Axum's automatic query parsing error (HTTP 400)
# Failed to deserialize query string: invalid digit found in string
```

---

## 📋 **Step 3: Implement `union_elements` Handler (30 minutes)**

### **3.1 Review the Models**

```rust
// Request body struct (JSON POST)
#[derive(Debug, Deserialize, ToSchema)]
pub struct UnionRequest {
    /// First element index
    pub element1: usize,
    /// Second element index
    pub element2: usize,
}

// Response struct
#[derive(Debug, Serialize, ToSchema)]
pub struct UnionResponse {
    /// Whether the elements were successfully merged (false if already connected)
    pub merged: bool,
    /// The new root of the set
    pub root: usize,
}
```

**What this means**:
- Client sends: `POST /unionfind/abc-123/union` with `{"element1": 3, "element2": 7}`
- Server returns: `{"merged": true, "root": 3}` (if they weren't connected before)
- Or: `{"merged": false, "root": 3}` (if they were already in the same set)

### **3.2 Understand the Endpoint**

**Endpoint**: `POST /api/v1/unionfind/{id}/union`  
**Purpose**: Union two elements together  
**Request**: Path parameter `{id}`, JSON body with `element1` and `element2`  
**Response**: 200 OK with union result  
**Errors**:
- 404 if instance not found
- 400 if either element is out of bounds

### **3.3 Implementation Walkthrough**

```rust
#[utoipa::path(
    post,
    path = "/api/v1/unionfind/{id}/union",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    request_body = UnionRequest,
    responses(
        (status = 200, description = "Union operation successful", body = UnionResponse),
        (status = 404, description = "Instance not found"),
        (status = 400, description = "Invalid element indices")
    ),
    tag = "Operations"
)]
pub async fn union_elements(
    State(state): State<AppState>,         // 1️⃣ Extract state
    Path(id): Path<Uuid>,                  // 2️⃣ Extract UUID
    Json(payload): Json<UnionRequest>,     // 3️⃣ Extract JSON body
) -> Response {
    // 4️⃣ Access instance and perform union operation
    // Get the union result and the actual root after the operation
    let result = state.get_instance(id, |uf| {
        uf.union(payload.element1, payload.element2)
            .and_then(|merged| {
                // After union, get the actual root
                uf.find(payload.element1).map(|root| (merged, root))
            })
    });

    match result {
        Some(Ok((merged, root))) => (
            // 5️⃣ Success: union completed
            StatusCode::OK,
            Json(UnionResponse {
                merged,
                root,
            }),
        ).into_response(),
        Some(Err(e)) => {
            // 6️⃣ Element out of bounds
            error_response(StatusCode::BAD_REQUEST, &e)
        }
        None => {
            // 7️⃣ Instance not found
            error_response(StatusCode::NOT_FOUND, "Instance not found")
        }
    }
}
```

### **3.4 Step-by-Step Explanation**

**1️⃣-2️⃣ Extract State and Path** (same as before)

**3️⃣ Extract JSON Body**:
```rust
Json(payload): Json<UnionRequest>
```
- Axum reads request body
- Deserializes to `UnionRequest { element1, element2 }`
- If JSON invalid or fields missing, Axum returns 400 automatically

**4️⃣ Perform Union Operation**:
```rust
let result = state.get_instance(id, |uf| {
    uf.union(payload.element1, payload.element2)
        .and_then(|merged| {
            // After union, get the actual root
            uf.find(payload.element1).map(|root| (merged, root))
        })
});
```

**How the union works**:
- `uf.union()` returns `Result<bool, Error>`
  - `Ok(true)` if elements were merged (they were in different sets)
  - `Ok(false)` if they were already connected (same set)
  - `Err(_)` if either element is out of bounds
- `.and_then()` chains the next operation only if union succeeded
- `uf.find()` gets the root after the union
- Final result: `Option<Result<(bool, usize), Error>>`

**Why this pattern is clean**:
- ✅ Single operation does both: union AND check connectivity
- ✅ Uses `and_then()` for clean error propagation
- ✅ No manual connectivity check needed (Mission 10's `union()` tells us)

**Root discovery**:
```rust
uf.find(payload.element1).map(|root| (merged, root))
```
- After union, both elements have the same root
- Find either one to get the new root
- Safe because union already validated the elements

**5️⃣ Success Case**:
```rust
Some(Ok((merged, root))) => (
    StatusCode::OK,
    Json(UnionResponse {
        merged,
        root,
    }),
).into_response()
```
- Union completed successfully
- `merged = true` means trees were actually merged
- `merged = false` means elements were already in same set
- Return HTTP 200 with `merged` status and new `root`

**6️⃣-7️⃣ Error Cases** (same pattern as find_element)

### **3.5 Activity: Test Union Operations**

```powershell
# Using the instance from Step 2.6 ($ID should still be set)

# Initially, check that elements 3 and 7 are separate
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=3" | ConvertFrom-Json
# Expected: {"element": 3, "root": 3}

curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=7" | ConvertFrom-Json
# Expected: {"element": 7, "root": 7}

# Union elements 3 and 7
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 3, "element2": 7}' | ConvertFrom-Json | ConvertTo-Json

# Expected output (root could be 3 or 7 depending on implementation):
# {
#   "merged": true,
#   "root": 3
# }

# Verify they now share the same root
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=3" | ConvertFrom-Json
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=7" | ConvertFrom-Json
# Both should return the same root (e.g., {"element": X, "root": 3})

# Try to union again (should show already_connected = true)
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 3, "element2": 7}' | ConvertFrom-Json | ConvertTo-Json

# Expected:
# {
#   "merged": false,
#   "root": 3
# }

# Test element out of bounds
curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 3, "element2": 99}'

# Expected: {"code":"400 Bad Request","message":"One or both elements (3, 99) are out of bounds"}

# Get stats to see components decreased
curl -s "http://localhost:8080/api/v1/unionfind/$ID/stats" | ConvertFrom-Json

# Expected (after one union of 10 elements):
# {"size": 10, "num_components": 9}
```

---

## 📋 **Step 4: Implement `check_connected` Handler (25 minutes)**

### **4.1 Review the Models**

```rust
// Query parameter struct (two elements)
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectedRequest {
    /// First element
    pub element1: usize,
    /// Second element
    pub element2: usize,
}

// Response struct
#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectedResponse {
    /// Whether the elements are in the same set
    pub connected: bool,
}
```

**What this means**:
- Client sends: `GET /unionfind/abc-123/connected?element1=3&element2=7`
- Server returns: `{"element1": 3, "element2": 7, "connected": true}`

### **4.2 Understand the Endpoint**

**Endpoint**: `GET /api/v1/unionfind/{id}/connected?element1=N&element2=M`  
**Purpose**: Check if two elements are in the same set  
**Request**: Path parameter `{id}`, query parameters `element1` and `element2`  
**Response**: 200 OK with connectivity status  
**Errors**:
- 404 if instance not found
- 400 if either element is out of bounds

### **4.3 Implementation Walkthrough**

```rust
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/connected",
    params(
        ("id" = Uuid, Path, description = "Instance ID"),
        ("element1" = usize, Query, description = "First element"),
        ("element2" = usize, Query, description = "Second element")
    ),
    responses(
        (status = 200, description = "Connectivity check successful", body = ConnectedResponse),
        (status = 404, description = "Instance not found"),
        (status = 400, description = "Invalid element indices")
    ),
    tag = "Operations"
)]
pub async fn check_connected(
    State(state): State<AppState>,             // 1️⃣ Extract state
    Path(id): Path<Uuid>,                      // 2️⃣ Extract UUID
    Query(params): Query<ConnectedRequest>,    // 3️⃣ Extract query params
) -> Response {
    // 4️⃣ Access instance and check connectivity
    let result = state.get_instance(id, |uf| {
        uf.connected(params.element1, params.element2)
    });

    match result {
        Some(Ok(connected)) => (
            // 5️⃣ Success: connectivity determined
            StatusCode::OK,
            Json(ConnectedResponse { connected }),
        ).into_response(),
        Some(Err(e)) => {
            // 6️⃣ Element out of bounds
            error_response(StatusCode::BAD_REQUEST, &e)
        }
        None => {
            // 7️⃣ Instance not found
            error_response(StatusCode::NOT_FOUND, "Instance not found")
        }
    }
}
```

### **4.4 Step-by-Step Explanation**

**1️⃣-2️⃣ Extract State and Path** (same as before)

**3️⃣ Extract Multiple Query Parameters**:
```rust
Query(params): Query<ConnectedRequest>
```
- From URL `/api/v1/unionfind/abc-123/connected?element1=3&element2=7`
- Axum parses query string `element1=3&element2=7`
- Deserializes to `ConnectedRequest { element1: 3, element2: 7 }`
- If either parameter missing or invalid, Axum returns 400

**4️⃣ Check Connectivity**:
```rust
let result = state.get_instance(id, |uf| {
    uf.connected(params.element1, params.element2)
});
```

**Using `connected()` method**:
- Mission 10's `UnionFind` provides a `connected()` method
- Returns `Result<bool, UnionFindError>`
- Internally finds roots of both elements and compares
- Returns error if either element is out of bounds

**Connectivity logic**:
- If elements share the same root → `Ok(true)`
- If elements have different roots → `Ok(false)`
- If either element invalid → `Err(error message)`

**5️⃣ Success Case**:
```rust
Some(Ok(connected)) => (
    StatusCode::OK,
    Json(ConnectedResponse { connected }),
).into_response()
```
- Both elements valid, connectivity determined
- Return HTTP 200 with boolean result

**6️⃣-7️⃣ Error Cases** (same pattern)

### **4.5 Activity: Test Connectivity Checks**

```powershell
# Continuing from Step 3.5 (elements 3 and 7 should be connected)

# Check connectivity of elements we unioned (3 and 7)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=7" | ConvertFrom-Json | ConvertTo-Json

# Expected:
# {
#   "connected": true
# }

# Check connectivity with a different element (e.g., 5)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=5" | ConvertFrom-Json | ConvertTo-Json

# Expected (not connected yet):
# {
#   "connected": false
# }

# Union element 5 with 3
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 3, "element2": 5}' | ConvertFrom-Json | ConvertTo-Json

# Now check connectivity again (should be connected)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=5&element2=7" | ConvertFrom-Json | ConvertTo-Json

# Expected (5 connected to 3, 3 connected to 7, so 5 connected to 7):
# {
#   "connected": true
# }

# Test element out of bounds
curl "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=99"

# Expected: {"code":"400 Bad Request","message":"One or both elements (3, 99) are out of bounds"}

# Test reflexivity (element connected to itself)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=3" | ConvertFrom-Json | ConvertTo-Json

# Expected (always true):
# {
#   "connected": true
# }
```

---

## 📋 **Step 5: Complete API Integration Testing (20 minutes)**

### **5.1 Full Workflow Test**

Now let's test all six endpoints together in a realistic workflow:

```powershell
# 1. Create a new Union-Find instance with 15 elements
Write-Host "=== STEP 1: CREATE INSTANCE ===" -ForegroundColor Cyan
$response = curl -s -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" `
  -d '{"size": 15}' | ConvertFrom-Json
$ID = $response.id
Write-Host "Created instance: $ID" -ForegroundColor Green
Write-Host ""

# 2. Get initial stats (should be 15 separate components)
Write-Host "=== STEP 2: INITIAL STATS ===" -ForegroundColor Cyan
curl -s "http://localhost:8080/api/v1/unionfind/$ID/stats" | ConvertFrom-Json | ConvertTo-Json
Write-Host ""

# 3. Find roots of elements (should each be their own root)
Write-Host "=== STEP 3: FIND INITIAL ROOTS ===" -ForegroundColor Cyan
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=0" | ConvertFrom-Json | ConvertTo-Json
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=5" | ConvertFrom-Json | ConvertTo-Json
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=10" | ConvertFrom-Json | ConvertTo-Json
Write-Host ""

# 4. Check initial connectivity (should be false)
Write-Host "=== STEP 4: CHECK INITIAL CONNECTIVITY ===" -ForegroundColor Cyan
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=0&element2=5" | ConvertFrom-Json | ConvertTo-Json
Write-Host ""

# 5. Perform several union operations
Write-Host "=== STEP 5: UNION OPERATIONS ===" -ForegroundColor Cyan
Write-Host "Union 0 and 1:" -ForegroundColor Yellow
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 0, "element2": 1}' | ConvertFrom-Json | ConvertTo-Json

Write-Host "Union 2 and 3:" -ForegroundColor Yellow
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 2, "element2": 3}' | ConvertFrom-Json | ConvertTo-Json

Write-Host "Union 0 and 2 (merges the two groups):" -ForegroundColor Yellow
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" `
  -d '{"element1": 0, "element2": 2}' | ConvertFrom-Json | ConvertTo-Json
Write-Host ""

# 6. Get updated stats (should have fewer components)
Write-Host "=== STEP 6: UPDATED STATS ===" -ForegroundColor Cyan
curl -s "http://localhost:8080/api/v1/unionfind/$ID/stats" | ConvertFrom-Json | ConvertTo-Json
Write-Host ""

# 7. Check connectivity after unions
Write-Host "=== STEP 7: CHECK CONNECTIVITY AFTER UNIONS ===" -ForegroundColor Cyan
Write-Host "Are 0 and 3 connected? (should be true):" -ForegroundColor Yellow
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=0&element2=3" | ConvertFrom-Json | ConvertTo-Json

Write-Host "Are 1 and 2 connected? (should be true):" -ForegroundColor Yellow
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=1&element2=2" | ConvertFrom-Json | ConvertTo-Json

Write-Host "Are 0 and 5 connected? (should be false):" -ForegroundColor Yellow
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=0&element2=5" | ConvertFrom-Json | ConvertTo-Json
Write-Host ""

# 8. Find roots after unions
Write-Host "=== STEP 8: FIND ROOTS AFTER UNIONS ===" -ForegroundColor Cyan
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=0" | ConvertFrom-Json | ConvertTo-Json
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=1" | ConvertFrom-Json | ConvertTo-Json
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=2" | ConvertFrom-Json | ConvertTo-Json
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=3" | ConvertFrom-Json | ConvertTo-Json
Write-Host "All four elements (0,1,2,3) should have the SAME root" -ForegroundColor Green
Write-Host ""

# 9. Delete the instance
Write-Host "=== STEP 9: DELETE INSTANCE ===" -ForegroundColor Cyan
curl -X DELETE "http://localhost:8080/api/v1/unionfind/$ID"
Write-Host "Instance deleted (no output = success)" -ForegroundColor Green
Write-Host ""

# 10. Verify deletion
Write-Host "=== STEP 10: VERIFY DELETION ===" -ForegroundColor Cyan
curl "http://localhost:8080/api/v1/unionfind/$ID/stats"
Write-Host ""

Write-Host "=== COMPLETE API TEST FINISHED ===" -ForegroundColor Green
```

### **5.2 Expected Workflow Output**

```
=== STEP 1: CREATE INSTANCE ===
Created instance: abc-123-def-456

=== STEP 2: INITIAL STATS ===
{
  "size": 15,
  "num_components": 15
}

=== STEP 3: FIND INITIAL ROOTS ===
{"element": 0, "root": 0}
{"element": 5, "root": 5}
{"element": 10, "root": 10}

=== STEP 4: CHECK INITIAL CONNECTIVITY ===
{
  "element1": 0,
  "element2": 5,
  "connected": false
}

=== STEP 5: UNION OPERATIONS ===
Union 0 and 1:
{
  "element1": 0,
  "element2": 1,
  "new_root": 0,
  "already_connected": false
}
Union 2 and 3:
{
  "element1": 2,
  "element2": 3,
  "new_root": 2,
  "already_connected": false
}
Union 0 and 2 (merges the two groups):
{
  "element1": 0,
  "element2": 2,
  "new_root": 0,
  "already_connected": false
}

=== STEP 6: UPDATED STATS ===
{
  "size": 15,
  "num_components": 12
}

=== STEP 7: CHECK CONNECTIVITY AFTER UNIONS ===
Are 0 and 3 connected? (should be true):
{"element1": 0, "element2": 3, "connected": true}
Are 1 and 2 connected? (should be true):
{"element1": 1, "element2": 2, "connected": true}
Are 0 and 5 connected? (should be false):
{"element1": 0, "element2": 5, "connected": false}

=== STEP 8: FIND ROOTS AFTER UNIONS ===
{"element": 0, "root": 0}
{"element": 1, "root": 0}
{"element": 2, "root": 0}
{"element": 3, "root": 0}
All four elements (0,1,2,3) should have the SAME root

=== STEP 9: DELETE INSTANCE ===
Instance deleted (no output = success)

=== STEP 10: VERIFY DELETION ===
{"code":"404 Not Found","message":"Union-Find instance abc-123... not found"}

=== COMPLETE API TEST FINISHED ===
```

---

## 📋 **Step 6: Testing with Swagger UI (15 minutes)**

### **6.1 Complete Swagger Workflow**

1. **Open Swagger UI**: http://localhost:8080/swagger-ui

2. **Create Instance**:
   - Expand "POST /api/v1/unionfind"
   - Try it out, set size to 10
   - Execute and **copy the instance ID**

3. **Get Stats**:
   - Expand "GET /api/v1/unionfind/{id}/stats"
   - Paste instance ID
   - Execute → should show 10 components

4. **Find Root**:
   - Expand "GET /api/v1/unionfind/{id}/find"
   - Paste instance ID, set element to 5
   - Execute → should return `{"element": 5, "root": 5}`

5. **Union Elements**:
   - Expand "POST /api/v1/unionfind/{id}/union"
   - Paste instance ID
   - Set body to `{"element1": 3, "element2": 7}`
   - Execute → should show union result with `already_connected: false`

6. **Check Connectivity**:
   - Expand "GET /api/v1/unionfind/{id}/connected"
   - Paste instance ID
   - Set element1=3, element2=7
   - Execute → should return `{"connected": true}`

7. **Verify Stats Updated**:
   - Go back to "GET /api/v1/unionfind/{id}/stats"
   - Execute → should now show 9 components (one less)

8. **Delete Instance**:
   - Expand "DELETE /api/v1/unionfind/{id}"
   - Paste instance ID
   - Execute → should return 204 No Content

### **6.2 Error Testing in Swagger**

**Test 400 Errors**:
- Try find with element=99 (out of bounds)
- Try union with element1=99
- Try connected with element1=99

**Test 404 Errors**:
- Use a fake UUID like "00000000-0000-0000-0000-000000000000"
- Try any operation → should get 404

**Test Validation**:
- Try to create with size=0 → 400
- Try find with element=-1 → Swagger won't let you (validation in UI)

---

## 📋 **Step 7: Understanding Query vs. Path vs. Body (10 minutes)**

### **7.1 When to Use Each Parameter Type**

| **Parameter Type** | **Use Case** | **Example** | **Extractor** |
|--------------------|--------------|-------------|---------------|
| **Path** | Resource identification | `/unionfind/{id}` | `Path<Uuid>` |
| **Query** | Optional filters, read operations | `/find?element=5` | `Query<T>` |
| **Body** | Complex data, write operations | `POST {"element1": 3, ...}` | `Json<T>` |

### **7.2 Why Different Patterns?**

**Find Root** (Query parameter):
```
GET /api/v1/unionfind/{id}/find?element=5
```
- ✅ GET request (semantically correct for read operation)
- ✅ Simple parameter (just one number)
- ✅ Can bookmark/share URL
- ✅ RESTful convention for queries

**Union Elements** (Body parameter):
```
POST /api/v1/unionfind/{id}/union
Body: {"element1": 3, "element2": 7}
```
- ✅ POST request (write operation)
- ✅ Multiple parameters in structured format
- ✅ Can extend later (e.g., add "weighted" option)
- ✅ Follows HTTP conventions (POST for mutations)

**Alternative design** (less idiomatic):
```
// NOT RECOMMENDED: Using POST with query params for mutation
POST /api/v1/unionfind/{id}/union?element1=3&element2=7

// NOT RECOMMENDED: Using GET with body for read
GET /api/v1/unionfind/{id}/find
Body: {"element": 5}
```

**Why body for union but query for find?**
- **Union is a mutation** → POST → body is conventional
- **Find is a query** → GET → GET requests shouldn't have bodies (HTTP spec)
- **Consistency matters** → Follow REST conventions for predictable API

### **7.3 Real-World API Patterns**

**GitHub API** (similar pattern):
```
GET /repos/{owner}/{repo}/pulls?state=open     # Query params for filtering
POST /repos/{owner}/{repo}/issues              # Body for creating
DELETE /repos/{owner}/{repo}/labels/{name}     # Path for identification
```

**Your Union-Find API**:
```
GET /unionfind/{id}/find?element=5             # Query param for read
POST /unionfind/{id}/union                      # Body for write
DELETE /unionfind/{id}                         # Path only for delete
```

---

## 📋 **Step 8: Document Your Understanding (15 minutes)**

### **8.1 Create Day 10 Notes**

Create `DAY10_NOTES.md`:

```markdown
# Day 10: Union/Find Operations Implementation Notes

## Query<T> Extractor Implemented

### Pattern
```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct FindRequest {
    pub element: usize,
}

async fn handler(
    Query(params): Query<FindRequest>
) -> Response {
    // params.element is guaranteed valid usize
}
```

### URL Parsing
- URL: `/find?element=5`
- Axum parses query string to struct
- Auto-validates types (returns 400 if invalid)
- Works with multiple parameters: `/connected?element1=3&element2=7`

## Handlers Implemented

### 1. find_element (GET /api/v1/unionfind/{id}/find?element=N)
- **Input**: UUID path param + element query param
- **Output**: 200 OK + FindResponse (element, root)
- **Errors**: 
  - 404 if instance not found
  - 400 if element out of bounds
- **Three-layer error model**:
  1. Type validation (Axum)
  2. Resource existence (AppState)
  3. Operation validation (UnionFind)

### 2. union_elements (POST /api/v1/unionfind/{id}/union)
- **Input**: UUID path param + JSON body (element1, element2)
- **Output**: 200 OK + UnionResponse (elements, new_root, already_connected)
- **Logic**: 
  - Check connectivity BEFORE union
  - Perform union operation
  - Find new root after union
- **Errors**: Same as find_element

### 3. check_connected (GET /api/v1/unionfind/{id}/connected?element1=N&element2=M)
- **Input**: UUID path param + two element query params
- **Output**: 200 OK + ConnectedResponse (connected)
- **Logic**: Uses Mission 10's `connected()` method which finds and compares roots
- **Errors**: Same as find_element

## Three-Layer Error Handling

### Layer 1: Type Validation (Automatic)
```rust
Query(query): Query<FindQuery>
// Axum validates query params match struct
// Returns 400 if parsing fails
```

### Layer 2: Resource Existence (Your Code)
```rust
match state.get_instance(id, |uf| ...) {
    None => error_response(404, "Instance not found"),
    Some(result) => // Continue to layer 3
}
```

### Layer 3: Operation Validation (Union-Find)
```rust
match result {
    Ok(value) => (200, Json(response)),
    Err(_) => error_response(400, "Element out of bounds"),
}
```

## Request Parameter Patterns

| **Operation** | **Method** | **Path Param** | **Query Param** | **Body** |
|---------------|------------|----------------|-----------------|----------|
| create | POST | - | - | ✅ size |
| delete | DELETE | ✅ id | - | - |
| stats | GET | ✅ id | - | - |
| find | GET | ✅ id | ✅ element | - |
| union | POST | ✅ id | - | ✅ element1, element2 |
| connected | GET | ✅ id | ✅ element1, element2 | - |

**Pattern**:
- **Mutations** (create, delete, union) → POST/DELETE with body
- **Queries** (stats, find, connected) → GET with query params
- **Resource ID** → Always path parameter

## Key Code Patterns

### ? Operator for Error Propagation
```rust
let root1 = uf.find(query.element1)?;  // Early return on error
let root2 = uf.find(query.element2)?;
Ok(root1 == root2)  // Only reaches here if both succeeded
```

### Connectivity Check Before Union
```rust
let already_connected = uf.find(element1).ok() == uf.find(element2).ok()
    && uf.find(element1).is_ok();
```
- Check if both have same root AND both are valid
- Tells client if union was redundant

### Safe Unwrap After Validation
```rust
match uf.union(element1, element2) {
    Ok(()) => {
        let new_root = uf.find(element1).unwrap();  // Safe!
        // Union succeeded, so element1 is valid
    }
    Err(e) => Err(e),
}
```

## Complete API Workflow Tested

```powershell
# 1. Create instance
$response = curl -s -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" -d '{"size": 15}' | ConvertFrom-Json
$ID = $response.id

# 2. Find initial roots (each element is its own root)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=0"

# 3. Check initial connectivity (should be false)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=0&element2=1"

# 4. Union elements
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" -d '{"element1": 0, "element2": 1}'

# 5. Verify connectivity (should be true now)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=0&element2=1"

# 6. Check stats (num_components decreased)
curl -s "http://localhost:8080/api/v1/unionfind/$ID/stats"

# 7. Delete instance
curl -X DELETE "http://localhost:8080/api/v1/unionfind/$ID"
```

## Tomorrow (Day 11)

- Complete Utoipa annotations for all endpoints
- Add request/response examples to OpenAPI spec
- Document edge cases and constraints
- Final Swagger UI polish
- Integration test suite (if time permits)
```

---

## ✅ **Day 10 Completion Checklist**

By now you should have:

- [x] **Understood Query<T> extractor**: Query parameter parsing and validation
- [x] **Implemented find_element**: GET handler with query params and three-layer errors
- [x] **Implemented union_elements**: POST handler with connectivity check
- [x] **Implemented check_connected**: GET handler with dual query params
- [x] **Tested all six endpoints**: Create, delete, stats, find, union, connected
- [x] **Understood parameter patterns**: When to use path vs. query vs. body
- [x] **Tested complete workflow**: Multi-step API usage scenario
- [x] **Created notes**: DAY10_NOTES.md documenting operations

## 🚫 **What You Should NOT Know Yet**

It's OK if you don't understand:
- ❌ Complete Utoipa annotations (Day 11)
- ❌ OpenAPI schema details (Day 11)
- ❌ Integration test patterns (advanced topic)
- ❌ Performance optimization (covered in Mission 10 core)

---

## 🎯 **Day 11 Preview**

Tomorrow you'll:
1. Complete Utoipa annotations for all endpoints
2. Add detailed request/response examples
3. Document constraints and validations in OpenAPI
4. Polish Swagger UI presentation
5. Review final API design

**For now**: REST! You've completed the FULL REST API implementation. All six endpoints are working!

---

## 📊 **Quick Reference**

### **Query Parameter Template**
```rust
// Define query struct
#[derive(Debug, Deserialize, ToSchema)]
pub struct MyRequest {
    pub param1: usize,
    pub param2: Option<String>,  // Optional parameter
}

// Use in handler
async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<MyRequest>,
) -> Response {
    println!("param1: {}", params.param1);
    if let Some(p2) = params.param2 {
        println!("param2: {}", p2);
    }
    // ...
}
```

### **Three-Layer Error Template**
```rust
match state.get_instance(id, |uf| {
    // Layer 3: Operation validation
    uf.some_operation(param)  // Returns Result<T, Error>
}) {
    Some(Ok(value)) => {
        // Success
        (StatusCode::OK, Json(response)).into_response()
    }
    Some(Err(_)) => {
        // Layer 3 error: Operation failed
        error_response(StatusCode::BAD_REQUEST, "...")
    }
    None => {
        // Layer 2 error: Resource not found
        error_response(StatusCode::NOT_FOUND, "...")
    }
}
// Layer 1 (Type validation) happens before handler runs (Axum)
```

### **Complete curl Test Suite**
```powershell
# Save this as test-api.ps1

# Create instance
$response = curl -s -X POST http://localhost:8080/api/v1/unionfind `
  -H "Content-Type: application/json" -d '{"size": 10}' | ConvertFrom-Json
$ID = $response.id

# Get stats
curl -s "http://localhost:8080/api/v1/unionfind/$ID/stats"

# Find root
curl -s "http://localhost:8080/api/v1/unionfind/$ID/find?element=5"

# Union elements
curl -s -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" `
  -H "Content-Type: application/json" -d '{"element1": 3, "element2": 7}'

# Check connected
curl -s "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=7"

# Delete instance
curl -X DELETE "http://localhost:8080/api/v1/unionfind/$ID"
```

---

**Day 10 Status**: ✅ All Union/Find Operation Endpoints Complete  
**Next**: Day 11 - Complete Utoipa Documentation  
**Reference**: [TUTORIAL.md](TUTORIAL.md) sections "API Design" and "Implementation"
