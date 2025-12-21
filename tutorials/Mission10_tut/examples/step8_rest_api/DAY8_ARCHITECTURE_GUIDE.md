# Day 8: REST API Architecture & Design - Step-by-Step Guide

**Mission 10 - Day 8 Activity**  
**Goal**: Understand the REST API architecture and tech stack (REQ-8) WITHOUT deep coding yet  
**Time**: 60-90 minutes  
**Prerequisites**: Understanding of HTML/CSS/JavaScript (traditional web stack)

---

## 🎯 **Today's Success Criteria**

By the end of Day 8, you should be able to:
- [ ] Explain the tech stack (Axum + Utoipa + Tokio)
- [ ] List all 6 API endpoints and their purposes
- [ ] Understand the file structure (what each file does)
- [ ] Explain the state management pattern (Arc<Mutex<HashMap>>)
- [ ] Map traditional web concepts to this Rust stack

**You will NOT yet**: Write handlers, use Axum extractors, or write Utoipa macros (that's Days 9-11)

---

## 🌐 **Traditional Web Stack → Rust Stack Mapping**

Since you know HTML/CSS/JavaScript, here's how this Rust stack compares:

| **Traditional Stack** | **Rust Stack (This Project)** | **Purpose** |
|-----------------------|-------------------------------|-------------|
| **Express.js** (Node.js) | **Axum** | Web framework handling HTTP requests/responses |
| **Swagger/OpenAPI** (manual YAML) | **Utoipa** | API documentation (auto-generated from code!) |
| **Node.js event loop** | **Tokio** | Async runtime for concurrent request handling |
| **JavaScript** | **Rust** | Programming language |
| **Frontend (HTML/CSS/JS)** | **Swagger UI** | Interactive API testing interface (auto-generated!) |

### **Key Difference: No Manual Frontend!**

**Traditional Web**:
```
Backend (Express.js) ─┐
                      ├─> Manually write OpenAPI YAML
Frontend (React)      ├─> Manually build UI forms
                      └─> Can get out of sync!
```

**This Rust Stack**:
```
Backend (Axum + Utoipa) ─┐
                         ├─> Auto-generates OpenAPI spec
                         ├─> Auto-generates Swagger UI
                         └─> Always in sync (compile-time!)
```

---

## 📋 **Step 1: Understand the Tech Stack (15 minutes)**

### **1.1 Axum - The Web Framework**

**What it replaces**: Express.js, Flask, FastAPI

**Example comparison**:

**Express.js (JavaScript)**:
```javascript
// Define route
app.post('/api/v1/unionfind', (req, res) => {
    const { size } = req.body;  // Parse JSON manually
    // ... handle request
    res.status(201).json({ id: uuid, size });
});
```

**Axum (Rust)**:
```rust
// Define route (you'll see this tomorrow on Day 9)
async fn create_instance(
    State(app_state): State<AppState>,     // Auto-extract state
    Json(payload): Json<CreateRequest>,    // Auto-parse JSON (type-safe!)
) -> Response {
    // ... handle request (guaranteed valid CreateRequest!)
}
```

**Key benefit**: Type safety - if request doesn't match `CreateRequest` shape, Axum rejects it automatically!

### **1.2 Utoipa - Auto Documentation**

**What it replaces**: Manually writing `swagger.yaml` or `openapi.json`

**Traditional approach** (manual):
```yaml
# openapi.yaml - manually write this
paths:
  /api/v1/unionfind:
    post:
      summary: Create Union-Find instance
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                size:
                  type: integer
```

**Utoipa approach** (auto-generated):
```rust
// models.rs - Write this ONCE
#[derive(ToSchema)]  // ← Macro generates OpenAPI schema!
pub struct CreateRequest {
    #[schema(example = 10)]
    pub size: usize,
}

// The YAML above is generated automatically at compile-time!
```

**Key benefit**: Docs are ALWAYS correct - they're generated from the code itself!

### **1.3 Tokio - Async Runtime**

**What it replaces**: Node.js event loop

**Why it matters**: Handles multiple API requests concurrently (like when 100 users hit your API simultaneously)

**You don't need to understand async deeply yet** - just know Tokio is running in the background managing concurrency.

---

## 📋 **Step 2: Explore the File Structure (20 minutes)**

### **2.1 Open the Project**

```bash
cd tutorials/Mission10_tut/examples/step8_rest_api
```

### **2.2 Understand Each File's Role**

Open each file and **SKIM** (don't study deeply yet):

#### **File 1: `src/models.rs`** (Request/Response Contracts)
```bash
code src/models.rs
```

**What to notice**:
- [ ] `CreateRequest` struct - what you POST to create instance
- [ ] `CreateResponse` struct - what server returns
- [ ] `#[derive(ToSchema)]` - generates OpenAPI schema
- [ ] `#[schema(example = 10)]` - example values for docs

**Traditional web analogy**: Like TypeScript interfaces defining API contracts

#### **File 2: `src/handlers.rs`** (API Endpoint Logic)
```bash
code src/handlers.rs
```

**What to notice** (don't understand every line yet):
- [ ] Function names match endpoints: `create_instance`, `union_elements`, `find_root`, etc.
- [ ] `#[utoipa::path(...)]` - documents each endpoint
- [ ] `async fn` - these run asynchronously
- [ ] Pattern matching on `Option<Result<T>>` for error handling

**Traditional web analogy**: Like route handlers in Express.js (`app.post('/route', handler)`)

#### **File 3: `src/state.rs`** (Shared Application State)
```bash
code src/state.rs
```

**What to notice**:
- [ ] `Arc<Mutex<HashMap<Uuid, UnionFind>>>` - stores multiple Union-Find instances
- [ ] `Arc` = Atomic Reference Counting (shared ownership)
- [ ] `Mutex` = Mutual exclusion lock (exclusive access)
- [ ] `HashMap` = UUID → UnionFind instance mapping

**Why this pattern?**:
- Multiple async requests need access to the same data
- Must prevent two requests from modifying the same instance simultaneously
- Like a shared database connection pool in traditional web apps

#### **File 4: `src/errors.rs`** (Error Responses)
```bash
code src/errors.rs
```

**What to notice**:
- [ ] `ErrorResponse` struct with `code` and `message`
- [ ] `error_response()` helper function
- [ ] Converts errors to HTTP status codes (400, 404, 500)

**Traditional web analogy**: Like Express.js error middleware

#### **File 5: `src/main.rs`** (Server Setup & Routing)
```bash
code src/main.rs
```

**What to notice**:
- [ ] `Router::new()` - sets up routes (like Express `app` object)
- [ ] `.route("/api/v1/unionfind", post(create_instance))` - maps URL to handler
- [ ] `SwaggerUi::new("/swagger-ui")` - serves auto-generated docs
- [ ] `axum::serve(listener, app)` - starts the server

**Traditional web analogy**: Like your `server.js` or `app.js` main file

#### **File 6: `src/openapi.rs`** (API Specification)
```bash
code src/openapi.rs
```

**What to notice**:
- [ ] `#[derive(OpenApi)]` - generates full OpenAPI spec
- [ ] Lists all handlers in `paths(...)` section
- [ ] Lists all models in `components(...)` section

**Traditional web analogy**: Auto-generates `swagger.yaml` from code annotations

---

## 📋 **Step 3: Understand the 6 API Endpoints (15 minutes)**

### **3.1 The Complete API Surface**

Study this table (from TUTORIAL.md):

| # | Method | Endpoint | Purpose | Traditional Web Analogy |
|---|--------|----------|---------|------------------------|
| 1 | POST | `/api/v1/unionfind` | Create new instance | Like creating a new shopping cart |
| 2 | POST | `/api/v1/unionfind/{id}/union` | Connect two elements | Like merging two groups |
| 3 | GET | `/api/v1/unionfind/{id}/find` | Find root of element | Like finding group leader |
| 4 | GET | `/api/v1/unionfind/{id}/connected` | Check if connected | Like checking if users are friends |
| 5 | GET | `/api/v1/unionfind/{id}/stats` | Get statistics | Like getting cart summary |
| 6 | DELETE | `/api/v1/unionfind/{id}` | Delete instance | Like deleting a cart |

### **3.2 RESTful Design Principles**

**Resource**: Union-Find instance (identified by UUID)

**Operations** (CRUD):
- **Create**: POST `/unionfind` (creates new resource, returns ID)
- **Read**: GET `/unionfind/{id}/stats` (reads resource state)
- **Update**: POST `/unionfind/{id}/union` (modifies resource)
- **Delete**: DELETE `/unionfind/{id}` (removes resource)

**Why use UUIDs instead of integers?**
- Unique across distributed systems
- Can't guess other users' IDs
- Standard practice for REST APIs

### **3.3 Activity: Map Endpoints to Use Cases**

Answer these questions (check understanding):

- [ ] **Q**: Which endpoint would you call FIRST to use the API?  
  **A**: POST `/api/v1/unionfind` (create an instance)

- [ ] **Q**: If you want to connect elements 3 and 7, which endpoint?  
  **A**: POST `/api/v1/unionfind/{id}/union` with JSON `{"element1": 3, "element2": 7}`

- [ ] **Q**: How do you check how many components exist?  
  **A**: GET `/api/v1/unionfind/{id}/stats` returns `num_components`

- [ ] **Q**: What HTTP status code for "instance not found"?  
  **A**: 404 Not Found

---

## 📋 **Step 4: Understand State Management (15 minutes)**

### **4.1 The Challenge**

**Problem**: Union-Find needs `&mut self` (exclusive mutable access), but web servers handle multiple concurrent requests.

**Scenario**:
```
Request 1: POST /unionfind/abc/union (element1=3, element2=7)
Request 2: GET  /unionfind/abc/find?element=3
Request 3: POST /unionfind/abc/union (element1=1, element2=2)

All three requests arrive SIMULTANEOUSLY!
```

How do you ensure they don't corrupt the Union-Find state?

### **4.2 The Solution: Arc<Mutex<HashMap>>**

**Full Type**: `Arc<Mutex<HashMap<Uuid, UnionFind>>>`

Let's break this down layer by layer:

#### **Layer 1: HashMap<Uuid, UnionFind>**
```rust
// Store multiple Union-Find instances
{
  "uuid-123": UnionFind(size=10),
  "uuid-456": UnionFind(size=5),
  "uuid-789": UnionFind(size=20),
}
```

**Why HashMap?**: Support multiple users/instances simultaneously

#### **Layer 2: Mutex<HashMap<...>>**
```rust
// Wrap HashMap in a lock
Mutex::new(hashmap)
```

**What Mutex does**:
- Ensures only ONE thread can access HashMap at a time
- Request 1 acquires lock → modifies data → releases lock
- Request 2 waits for lock → acquires lock → reads data → releases lock

**Traditional web analogy**: Like database row-level locking

#### **Layer 3: Arc<Mutex<...>>**
```rust
// Wrap Mutex in shared ownership
Arc::new(Mutex::new(hashmap))
```

**What Arc does**:
- Allows multiple async tasks to share ownership
- When you `.clone()` Arc, you get a reference to SAME data
- Reference counting tracks how many tasks have access

**Traditional web analogy**: Like passing the same database connection pool to all route handlers

### **4.3 How It Works in Practice**

```rust
// main.rs - Create shared state
let state = AppState::new();  // Arc<Mutex<HashMap>> inside

// Clone state for each route (cheap - just increments reference count)
.route("/api/v1/unionfind", post(create_instance))
    .with_state(state.clone())  // ← Each handler gets a clone
.route("/api/v1/unionfind/:id/union", post(union_elements))
    .with_state(state.clone())  // ← All clones point to SAME data
```

When request arrives:
1. Axum extracts `State(app_state)` (gives handler an Arc clone)
2. Handler calls `app_state.get_instance(id, |uf| ...)`
3. Inside `get_instance`: acquires Mutex lock
4. Closure executes with `&mut UnionFind`
5. Lock released automatically (RAII)

---

## 📋 **Step 5: Document Your Understanding (15 minutes)**

### **5.1 Create Architecture Notes**

Create a new file documenting what you learned:

```bash
# Create notes file
code DAY8_NOTES.md
```

**What to write** (template):

```markdown
# Day 8: REST API Architecture Notes

## Tech Stack Decision (REQ-8)

### Axum
- **Purpose**: Web framework
- **Why chosen**: Type-safe extractors, built on Tokio
- **Compares to**: Express.js (Node.js)

### Utoipa
- **Purpose**: OpenAPI/Swagger documentation generator
- **Why chosen**: Compile-time doc generation, always in sync
- **Compares to**: Manual swagger.yaml writing

### Tokio
- **Purpose**: Async runtime
- **Why chosen**: Efficient concurrent request handling
- **Compares to**: Node.js event loop

## API Endpoints (REQ-8)

1. **POST /api/v1/unionfind** - Create instance
2. **POST /api/v1/unionfind/{id}/union** - Union operation
3. **GET /api/v1/unionfind/{id}/find** - Find root
4. **GET /api/v1/unionfind/{id}/connected** - Check connectivity
5. **GET /api/v1/unionfind/{id}/stats** - Get statistics
6. **DELETE /api/v1/unionfind/{id}** - Delete instance

## File Structure

- `main.rs` - Server setup, routing (orchestration)
- `handlers.rs` - Endpoint implementations (business logic)
- `models.rs` - Request/Response contracts (API interfaces)
- `state.rs` - Shared state management (Arc<Mutex<HashMap>>)
- `errors.rs` - Error handling (HTTP status codes)
- `openapi.rs` - API specification generation

## State Management Pattern

**Pattern**: `Arc<Mutex<HashMap<Uuid, UnionFind>>>`

- **Arc**: Shared ownership across async tasks
- **Mutex**: Exclusive access for mutations (one thread at a time)
- **HashMap**: Multiple Union-Find instances by UUID

**Why needed**: Web servers handle concurrent requests, but Union-Find needs exclusive mutable access.

## Key Questions Answered

- [x] What is the tech stack?
- [x] Why were these technologies chosen?
- [x] What are the 6 endpoints?
- [x] What does each file do?
- [x] How is concurrent access managed?

## Next Steps (Day 9+)

- Day 9: Implement handlers with Axum extractors
- Day 10: Complete all endpoint implementations
- Day 11: Add Utoipa annotations for OpenAPI
```

### **5.2 Answer Self-Check Questions**

Test your understanding:

- [ ] **Without looking**: Name the 3 main crates in the tech stack  
  **Answer**: Axum (web framework), Utoipa (OpenAPI), Tokio (async runtime)

- [ ] **Without looking**: What does Arc stand for and why is it needed?  
  **Answer**: Atomic Reference Counting - allows shared ownership across async tasks

- [ ] **Without looking**: What HTTP method creates a new Union-Find instance?  
  **Answer**: POST

- [ ] **Without looking**: What file contains the route definitions?  
  **Answer**: `src/main.rs`

---

## 📋 **Step 6: Optional Exploration (If Time Permits)**

### **6.1 Run the Server (Just to See It)**

```bash
cargo run
```

Expected output:
```
🚀 Starting Union-Find REST API...
✅ Server listening on http://127.0.0.1:8080
📚 Swagger UI: http://127.0.0.1:8080/swagger-ui
```

### **6.2 Visit Swagger UI**

Open browser: http://localhost:8080/swagger-ui

**What to notice**:
- [ ] All 6 endpoints listed
- [ ] Request/response examples
- [ ] "Try it out" buttons
- [ ] Schema definitions

**Remember**: This ENTIRE webpage was auto-generated from your Rust code!

### **6.3 Try One Request (Optional)**

In Swagger UI:
1. Expand "POST /api/v1/unionfind"
2. Click "Try it out"
3. Leave default `{"size": 10}`
4. Click "Execute"
5. See response with UUID

**Don't go deeper yet** - tomorrow (Day 9) you'll implement these handlers yourself!

---

## ✅ **Day 8 Completion Checklist**

By now you should have:

- [x] **Understood tech stack**: Axum (web), Utoipa (docs), Tokio (async)
- [x] **Mapped to traditional web**: Express.js → Axum, manual YAML → Utoipa
- [x] **Explored all 6 files**: Know what each file's purpose is
- [x] **Memorized 6 endpoints**: Can list them and their purposes
- [x] **Understood state management**: Arc<Mutex<HashMap>> pattern and why
- [x] **Created notes**: DAY8_NOTES.md documenting architecture
- [x] **Optional**: Seen Swagger UI in action

## 🚫 **What You Should NOT Know Yet**

It's OK if you don't understand:
- ❌ How to write Axum handlers (Day 9)
- ❌ What extractors are exactly (Day 9)
- ❌ Utoipa macro syntax (Day 11)
- ❌ Async/await details (covered in Rustaceans Ch17)
- ❌ How to implement endpoints from scratch (Days 9-10)

---

## 🎯 **Day 9 Preview**

Tomorrow you'll:
1. Understand Axum extractors (State, Path, Json, Query)
2. Implement your first handler (`create_instance`)
3. Learn request/response patterns
4. Test endpoints with curl

**For now**: REST! You've completed Day 8's architecture phase. Tomorrow you start coding!

---

**Day 8 Status**: ✅ Architecture & Design Phase Complete  
**Next**: Day 9 - Core REST API Implementation  
**Reference**: [TUTORIAL.md](TUTORIAL.md) sections "Part 1: Understanding the Technology Stack"
