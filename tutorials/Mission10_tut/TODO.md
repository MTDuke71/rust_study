# Mission 10 Tutorial: Union-Find - TODO List

**Last Updated**: October 28, 2024
**Tutorial Status**: Step 1 Complete

---

## 📋 Tutorial Overview

This tutorial teaches Union-Find through a 7-step progressive learning path, building from basic concepts to advanced applications.

**Learning Philosophy**: Start with unoptimized implementation to understand the problem, then add optimizations one at a time to see their impact.

---

## ✅ Completed

### Step 1: Basic Union-Find (No Optimizations)
- [x] Create step1_basic_union_find.rs
- [x] Implement BasicUnionFind struct
- [x] Basic find() operation (O(n) worst case)
- [x] Basic union() operation (O(n) worst case)
- [x] Tree visualization examples
- [x] Performance analysis section
- [x] Network connectivity demonstration
- [x] Exercises included in comments
- [x] Comprehensive documentation (400+ lines)
- [x] Successfully compiles and runs

**Key Concepts Taught**:
- Disjoint set data structure fundamentals
- Parent pointer representation
- Tree structure for sets
- Why optimization is needed (O(n) chains)

### Step 2: Path Compression Optimization ✅ COMPLETE
- [x] Create examples/step2_path_compression.rs
- [x] Copy BasicUnionFind from Step 1 as baseline
- [x] Implement PathCompressionUF struct
- [x] Modify find() to flatten tree during traversal
- [x] Add visualization showing compression effect
- [x] Comprehensive documentation (400+ lines)
- [x] All tests passing (3/3)
- [x] Successfully compiles and runs

**Key Concepts Taught**:
- Path compression optimization technique
- Two-pass find() algorithm
- Tree flattening during traversal
- Performance improvement from O(n) to O(log n)

### Step 3: Union by Rank Optimization ✅ COMPLETE
- [x] Create examples/step3_union_by_rank.rs
- [x] Implement UnionByRankUF struct
- [x] Add rank tracking array
- [x] Modify union() to attach smaller tree to larger
- [x] Add visualization showing balanced trees
- [x] Explain rank concept and properties
- [x] Side-by-side comparison with naive union
- [x] Interactive tree building demonstration
- [x] Comprehensive documentation (550+ lines)
- [x] All tests passing (3/3)
- [x] Successfully compiles and runs

**Key Concepts Taught**:
- Union by rank optimization technique
- Rank as upper bound on tree height
- Balanced tree construction
- Performance guarantee: tree height ≤ log₂(n)

### Step 4: Combined Optimizations ✅ COMPLETE
- [x] Create examples/step4_combined_optimizations.rs
- [x] Implement OptimizedUnionFind struct
- [x] Combine path compression + union by rank
- [x] Show synergy between optimizations
- [x] Progressive comparison of all 4 versions
- [x] Explain inverse Ackermann function α(n)
- [x] Real-world performance analysis
- [x] Connection to Mission 10 implementation
- [x] Comprehensive documentation (650+ lines)
- [x] All tests passing (5/5)
- [x] Successfully compiles and runs

**Key Concepts Taught**:
- Synergistic effect of combined optimizations
- O(α(n)) amortized complexity
- Industry-standard Union-Find implementation
- Practical performance characteristics

---

### Step 5: Real-World Applications ✅ COMPLETE
- [x] Create examples/step5_applications.rs
- [x] Implement 5 different applications
- [x] Use optimized Union-Find from Step 4
- [x] Include visualizations for each application
- [x] Comprehensive documentation (700+ lines)
- [x] All tests passing (5/5)
- [x] Successfully compiles and runs

**Key Concepts Taught**:
- Kruskal's Minimum Spanning Tree algorithm
- Connected components in graphs
- Cycle detection in undirected graphs
- Social network friend circles modeling
- Image segmentation by color similarity
- Performance comparison with alternative approaches
- When to use Union-Find vs DFS/BFS

**Applications Implemented**:

#### 5.1: Kruskal's Minimum Spanning Tree ✅
- [x] Implement Kruskal's algorithm
- [x] Use Union-Find for cycle detection
- [x] Demonstrate on weighted graph
- [x] Visualize MST construction
- [x] Explain why Union-Find is perfect here

#### 5.2: Connected Components in Graphs ✅
- [x] Given undirected graph, find all components
- [x] Use Union-Find to group nodes
- [x] Handle dynamic edge additions
- [x] Count components efficiently
- [x] Compare with DFS/BFS approaches

#### 5.3: Cycle Detection ✅
- [x] Detect cycles in undirected graphs
- [x] Use Union-Find to track connectivity
- [x] Add edges one by one
- [x] Report when cycle forms
- [x] Explain O(E·α(V)) complexity

#### 5.4: Social Network Friend Circles ✅
- [x] Model friend relationships
- [x] Find mutually connected groups
- [x] Add/query friendships dynamically
- [x] Count number of friend circles
- [x] Interactive demonstration

#### 5.5: Image Segmentation ✅
- [x] Segment image by color similarity
- [x] Treat pixels as elements
- [x] Union adjacent similar pixels
- [x] Find connected regions
- [x] Demonstrate on sample image (ASCII art)

**Content Delivered**:
- Introduction to applications (40 lines)
- Application 1: Kruskal's MST (180 lines)
- Application 2: Connected Components (140 lines)
- Application 3: Cycle Detection (120 lines)
- Application 4: Social Networks (140 lines)
- Application 5: Image Segmentation (160 lines)
- Performance comparison table (60 lines)
- 6 exercises with clear descriptions (60 lines)

**Exercises Included**:
- [x] Exercise 1: Implement percolation simulation
- [x] Exercise 2: Dynamic connectivity with deletions
- [x] Exercise 3: Least Common Ancestor queries
- [x] Exercise 4: Maze generation
- [x] Exercise 5: Number of Islands (LeetCode 200)
- [x] Exercise 6: Accounts Merge (LeetCode 721)

---

## 🚧 Step 6: Advanced Variants

**Target**: Explore Union-Find extensions and variants

### Implementation Tasks
- [ ] Create examples/step6_advanced_variants.rs
- [ ] Implement 3-4 Union-Find variants
- [ ] Compare with standard implementation
- [ ] Discuss trade-offs

### Variants to Implement

#### 6.1: Weighted Union-Find
- [ ] Add weight/distance tracking
- [ ] Implement weighted_union(x, y, weight)
- [ ] Query distance between elements
- [ ] Application: network latency
- [ ] Handle negative weights (if possible)

#### 6.2: Union-Find with Undo
- [ ] Add operation history stack
- [ ] Implement undo() operation
- [ ] Support partial rollback
- [ ] Discuss memory overhead
- [ ] Show example use case

#### 6.3: Persistent Union-Find
- [ ] Immutable version with functional updates
- [ ] Path copying approach
- [ ] Multiple versions simultaneously
- [ ] Complexity analysis
- [ ] Use cases for persistence

#### 6.4: Union-Find with Customizable Merge
- [ ] Generic merge function for set data
- [ ] Track additional metadata per set
- [ ] Example: sum of elements in set
- [ ] Example: max element in set
- [ ] Application flexibility

### Content Structure
```rust
// Step 6 File Structure:
1. Introduction to variants (50 lines)
2. Variant 1: Weighted Union-Find (180 lines)
3. Variant 2: Union-Find with Undo (150 lines)
4. Variant 3: Persistent Union-Find (160 lines)
5. Variant 4: Customizable Merge (140 lines)
6. Comparison and trade-offs (80 lines)
7. Exercises (40 lines)
```

### Specific Content
- [ ] Each variant should include:
  - [ ] Motivation and use cases
  - [ ] Complete implementation
  - [ ] Comparison with standard version
  - [ ] Complexity analysis
  - [ ] Memory overhead
  - [ ] Example usage

- [ ] **Trade-offs Section**
  - [ ] Memory vs features matrix
  - [ ] Performance comparison
  - [ ] When to use which variant
  - [ ] Implementation complexity

- [ ] **Exercises**
  - [ ] Exercise 1: Implement randomized Union-Find
  - [ ] Exercise 2: Thread-safe concurrent Union-Find
  - [ ] Exercise 3: Union-Find with deletions
  - [ ] Exercise 4: Create hybrid variant

---

## 🚧 Step 7: Problem Solving Patterns

**Target**: Teach how to recognize and solve Union-Find problems

### Implementation Tasks
- [ ] Create examples/step7_problem_solving.rs
- [ ] Include 8-10 interview/competition problems
- [ ] Show step-by-step solutions
- [ ] Explain pattern recognition
- [ ] Add difficulty ratings

### Problems to Include

#### 7.1: Classic Problems
- [ ] **Number of Islands** (LeetCode 200)
  - [ ] Problem statement
  - [ ] Union-Find solution
  - [ ] Alternative approaches
  - [ ] Time/space complexity

- [ ] **Friend Circles** (LeetCode 547)
  - [ ] Problem statement
  - [ ] Union-Find approach
  - [ ] Handle adjacency matrix
  - [ ] Optimization tips

- [ ] **Redundant Connection** (LeetCode 684)
  - [ ] Problem statement
  - [ ] Cycle detection with Union-Find
  - [ ] Edge case handling
  - [ ] Complete solution

#### 7.2: Intermediate Problems
- [ ] **Accounts Merge** (LeetCode 721)
- [ ] **Most Stones Removed** (LeetCode 947)
- [ ] **Satisfiability of Equality Equations** (LeetCode 990)
- [ ] **Smallest String With Swaps** (LeetCode 1202)

#### 7.3: Advanced Problems
- [ ] **Number of Islands II** (LeetCode 305) - Dynamic connectivity
- [ ] **Evaluate Division** (LeetCode 399) - Weighted Union-Find
- [ ] **Checking Existence of Edge Length Limited Paths** (LeetCode 1697)

### Content Structure
```rust
// Step 7 File Structure:
1. Introduction to problem patterns (60 lines)
2. Pattern recognition guide (100 lines)
3. Problem 1-3: Classic (300 lines)
4. Problem 4-7: Intermediate (400 lines)
5. Problem 8-10: Advanced (400 lines)
6. Interview tips (80 lines)
7. Practice problems (60 lines)
```

### Specific Content
- [ ] **Pattern Recognition Section**
  - [ ] When to use Union-Find (checklist)
  - [ ] Common problem keywords
  - [ ] Identifying dynamic connectivity
  - [ ] vs DFS/BFS decision guide

- [ ] **Each Problem Should Include**:
  - [ ] Problem statement (clear and concise)
  - [ ] Input/output examples
  - [ ] Intuition and approach
  - [ ] Step-by-step solution
  - [ ] Complete working code
  - [ ] Complexity analysis
  - [ ] Common mistakes
  - [ ] Alternative approaches

- [ ] **Interview Tips Section**
  - [ ] How to explain Union-Find in interview
  - [ ] Common follow-up questions
  - [ ] Optimization discussion points
  - [ ] Edge cases to consider
  - [ ] Testing strategy

- [ ] **Practice Problems**
  - [ ] 10 additional problems with hints
  - [ ] Difficulty progression
  - [ ] Mix of problem types
  - [ ] Links to online judges

---

## 🌐 Step 8: REST API with OpenAPI/Swagger Documentation

**Target**: Demonstrate Union-Find as a production REST API service with comprehensive OpenAPI documentation

### Overview
Mission 10's Union-Find will be exposed as a RESTful web service with automatic OpenAPI/Swagger documentation, demonstrating professional API design and documentation standards.

### Implementation Tasks

#### 8.1: REST API Design
- [ ] Design RESTful endpoints for Union-Find operations
- [ ] Define request/response schemas
- [ ] Plan error handling and status codes
- [ ] Document API versioning strategy

**Proposed Endpoints**:
```
POST   /api/v1/unionfind/new          - Create new Union-Find instance
POST   /api/v1/unionfind/{id}/union   - Union two elements
GET    /api/v1/unionfind/{id}/find    - Find root of element
GET    /api/v1/unionfind/{id}/connected - Check if elements connected
GET    /api/v1/unionfind/{id}/components - Get all connected components
GET    /api/v1/unionfind/{id}/stats   - Get statistics (tree heights, compression rate)
DELETE /api/v1/unionfind/{id}         - Delete instance
```

#### 8.2: Technology Stack Selection
- [ ] Choose web framework (Axum, Actix-web, or Rocket)
- [ ] Select OpenAPI crate (`utoipa` recommended)
- [ ] Plan async runtime (Tokio)
- [ ] Design state management (Arc<Mutex<>> or parking_lot)

#### 8.3: Core Implementation
- [ ] Create `examples/step8_rest_api/` directory structure
- [ ] Implement web server with chosen framework
- [ ] Add Union-Find state management
- [ ] Implement all REST endpoints
- [ ] Add request validation
- [ ] Implement proper error responses

```rust
// examples/step8_rest_api/main.rs structure:
// 1. Dependencies and imports (30 lines)
// 2. OpenAPI schema definitions (100 lines)
// 3. State management structures (50 lines)
// 4. Endpoint handlers (200 lines)
// 5. OpenAPI integration (50 lines)
// 6. Server setup and main (40 lines)
```

#### 8.4: OpenAPI/Swagger Integration
- [ ] Add `utoipa` crate to Cargo.toml
- [ ] Add `utoipa-swagger-ui` for interactive documentation
- [ ] Annotate all endpoints with OpenAPI macros
- [ ] Define request/response schemas with examples
- [ ] Add operation descriptions and tags
- [ ] Configure Swagger UI endpoints

**OpenAPI Annotations Example**:
```rust
#[utoipa::path(
    post,
    path = "/api/v1/unionfind/{id}/union",
    tag = "Union-Find Operations",
    request_body = UnionRequest,
    responses(
        (status = 200, description = "Union successful", body = UnionResponse),
        (status = 404, description = "Union-Find instance not found"),
        (status = 400, description = "Invalid input")
    )
)]
async fn union_handler(...) { }
```

#### 8.5: Request/Response Models
- [ ] Define all data models with Serialize/Deserialize
- [ ] Add OpenAPI schema annotations
- [ ] Include validation constraints
- [ ] Provide example values
- [ ] Document all fields

**Models to Define**:
```rust
// Request models
- NewUnionFindRequest { size: usize, optimization: OptimizationType }
- UnionRequest { element1: usize, element2: usize }
- FindRequest { element: usize }
- ConnectedRequest { element1: usize, element2: usize }

// Response models
- NewUnionFindResponse { id: String, size: usize }
- UnionResponse { success: bool, new_root: usize }
- FindResponse { root: usize, path_length: usize }
- ConnectedResponse { connected: bool, component_size: usize }
- ComponentsResponse { count: usize, components: Vec<Vec<usize>> }
- StatsResponse { operations: u64, tree_heights: Vec<usize>, avg_depth: f64 }
- ErrorResponse { code: String, message: String }
```

#### 8.6: Interactive Documentation
- [ ] Set up Swagger UI at `/swagger-ui/`
- [ ] Set up OpenAPI spec endpoint at `/api-docs/openapi.json`
- [ ] Add ReDoc alternative view (optional)
- [ ] Include "Try it out" functionality
- [ ] Add authentication examples (if applicable)

#### 8.7: Testing and Examples
- [ ] Create `examples/step8_rest_api/README.md`
- [ ] Add curl examples for all endpoints
- [ ] Provide Postman collection (exported)
- [ ] Write integration tests
- [ ] Add load testing examples (optional)

**Example Usage Section**:
```bash
# Create new Union-Find instance
curl -X POST http://localhost:8080/api/v1/unionfind/new \
  -H "Content-Type: application/json" \
  -d '{"size": 10, "optimization": "PathCompressionAndRank"}'

# Union two elements
curl -X POST http://localhost:8080/api/v1/unionfind/{id}/union \
  -H "Content-Type: application/json" \
  -d '{"element1": 3, "element2": 7}'

# Check connectivity
curl -X GET "http://localhost:8080/api/v1/unionfind/{id}/connected?element1=3&element2=7"
```

#### 8.8: Advanced Features
- [ ] Rate limiting middleware
- [ ] Request/response logging
- [ ] Metrics endpoint (Prometheus format)
- [ ] Health check endpoint
- [ ] CORS configuration
- [ ] API versioning demonstration

#### 8.9: Documentation Content
- [ ] Create comprehensive `OPENAPI_GUIDE.md`
- [ ] Explain REST API design principles
- [ ] Document OpenAPI specification structure
- [ ] Provide Swagger UI usage guide
- [ ] Include API client generation examples
- [ ] Discuss production deployment considerations

### Content Structure
```
examples/step8_rest_api/
├── Cargo.toml                 - Dependencies (utoipa, axum, tokio, etc.)
├── README.md                  - Quick start guide
├── OPENAPI_GUIDE.md          - Comprehensive OpenAPI documentation
├── src/
│   ├── main.rs               - Server entry point
│   ├── models.rs             - Request/response models with OpenAPI schemas
│   ├── handlers.rs           - Endpoint handlers
│   ├── state.rs              - State management
│   ├── errors.rs             - Error handling
│   └── openapi.rs            - OpenAPI configuration
├── tests/
│   ├── integration_tests.rs  - API integration tests
│   └── load_tests.rs         - Performance testing (optional)
├── postman/
│   └── Mission10_UnionFind.postman_collection.json
└── examples/
    └── client_demo.rs        - Example API client
```

### Specific Content

- [ ] **OPENAPI_GUIDE.md Sections**:
  - [ ] Introduction to REST APIs
  - [ ] OpenAPI/Swagger overview
  - [ ] Why document APIs
  - [ ] Understanding the OpenAPI spec
  - [ ] Using Swagger UI
  - [ ] Generating client libraries
  - [ ] Best practices for API design
  - [ ] Union-Find API design rationale

- [ ] **Code Documentation**:
  - [ ] Inline comments explaining OpenAPI annotations
  - [ ] Examples of different HTTP status codes
  - [ ] Error handling patterns
  - [ ] State management explanation
  - [ ] Async/await patterns
  - [ ] Thread-safety considerations

- [ ] **Testing Documentation**:
  - [ ] How to run the server
  - [ ] Testing with curl
  - [ ] Using Swagger UI for testing
  - [ ] Writing integration tests
  - [ ] Performance considerations

### Learning Objectives
After completing Step 8, students should understand:
- [ ] RESTful API design principles
- [ ] OpenAPI specification format
- [ ] How to use utoipa for Rust API documentation
- [ ] Swagger UI for interactive documentation
- [ ] Request/response modeling
- [ ] Error handling in web services
- [ ] State management in async Rust
- [ ] API testing strategies
- [ ] Production deployment considerations

### Integration Points
- [ ] Reference from Mission 10 main README
- [ ] Link from daily study (Week 6)
- [ ] Cross-reference with advanced_examples/
- [ ] Zettelkasten page: "REST API Design Patterns"
- [ ] Zettelkasten page: "OpenAPI Best Practices"

### Quality Standards
- [ ] Server runs without warnings
- [ ] All endpoints functional and tested
- [ ] OpenAPI spec validates correctly
- [ ] Swagger UI loads and works properly
- [ ] Comprehensive error handling
- [ ] Thread-safe state management
- [ ] Clear documentation (300-500 lines)
- [ ] Production-ready code patterns
- [ ] Security considerations addressed

### Exercises
- [ ] **Exercise 1**: Add authentication to the API (JWT or API keys)
- [ ] **Exercise 2**: Implement API versioning (v1 vs v2)
- [ ] **Exercise 3**: Add caching layer (Redis) for frequently accessed data
- [ ] **Exercise 4**: Generate Python client library from OpenAPI spec
- [ ] **Exercise 5**: Implement WebSocket endpoint for real-time updates
- [ ] **Exercise 6**: Add GraphQL alternative endpoint
- [ ] **Exercise 7**: Containerize the API (Dockerfile + docker-compose)
- [ ] **Exercise 8**: Deploy to cloud platform (Fly.io, Railway, or AWS)

### Timeline Estimate
- **API Design**: 0.5 day
- **Core Implementation**: 2 days
- **OpenAPI Integration**: 1 day
- **Testing & Documentation**: 1 day
- **Advanced Features**: 1 day (optional)

**Total**: 4.5-5.5 days (November 22-27)

---

## 📚 Supporting Materials

### Exercises Directory
- [ ] Create exercises/README.md with all exercises
- [ ] exercises/step2_exercises.rs - Path compression exercises
- [ ] exercises/step3_exercises.rs - Union by rank exercises
- [ ] exercises/step4_exercises.rs - Combined optimization exercises
- [ ] exercises/step5_exercises.rs - Application exercises
- [ ] exercises/step6_exercises.rs - Variant implementation exercises
- [ ] exercises/step7_exercises.rs - Problem solving exercises

### Solutions Directory
- [ ] Create solutions/ directory for exercise answers
- [ ] solutions/step2_solutions.rs
- [ ] solutions/step3_solutions.rs
- [ ] solutions/step4_solutions.rs
- [ ] solutions/step5_solutions.rs
- [ ] solutions/step6_solutions.rs
- [ ] solutions/step7_solutions.rs

### Documentation
- [ ] Enhance README.md with:
  - [ ] Complete tutorial roadmap
  - [ ] Learning objectives for each step
  - [ ] Time estimates per step
  - [ ] Prerequisites
  - [ ] How to use the tutorial
  - [ ] Additional resources

- [ ] Create LEARNING_PATH.md
  - [ ] Suggested study schedule
  - [ ] Daily learning goals
  - [ ] Review checkpoints
  - [ ] Self-assessment questions

### Testing
- [ ] Create tests/ directory
- [ ] tests/step2_tests.rs - Test path compression
- [ ] tests/step3_tests.rs - Test union by rank
- [ ] tests/step4_tests.rs - Test combined optimizations
- [ ] Ensure all tutorial examples compile
- [ ] Add CI check for tutorial compilation

---

## 🎯 Quality Standards

Each tutorial step must meet:
- [ ] Compiles without warnings
- [ ] Runs successfully with clear output
- [ ] Comprehensive documentation (150-400 lines)
- [ ] Visual aids (ASCII art, diagrams)
- [ ] Exercises with varying difficulty
- [ ] Performance analysis section
- [ ] Comparison with previous steps
- [ ] Real-world examples
- [ ] Clear learning objectives stated

---

## 📊 Progress Tracking

### Timeline Estimate
- **Step 1**: ✅ Complete (October 28, 2024)
- **Step 2**: 1-2 days (November 3-4)
- **Step 3**: 1-2 days (November 5-6)
- **Step 4**: 2-3 days (November 7-9)
- **Step 5**: 2-3 days (November 10-12)
- **Step 6**: 2-3 days (November 13-15)
- **Step 7**: 3-4 days (November 16-19)
- **Supporting Materials**: 1-2 days (November 20-21)

**Total Estimated Time**: 12-18 days for complete tutorial series

### Current Status
- [x] Step 1: Complete ✅
- [x] Step 2: Complete ✅ (November 5, 2024)
- [x] Step 3: Complete ✅ (November 5, 2024)
- [x] Step 4: Complete ✅ (November 5, 2024)
- [ ] Step 5: Not started (90%)
- [ ] Step 6: Not started (95%)
- [ ] Step 7: Not started (100%)

---

## 🔗 Integration

### Zettelkasten Links
- [ ] Create zettelkasten/mission-10-tutorial.md
- [ ] Link to mission-10.md
- [ ] Link to daily study materials
- [ ] Link to relevant graph algorithm notes

### Cross-References
- [ ] Reference from Mission10/README.md
- [ ] Update tutorials/README.md
- [ ] Link from Week 6 daily study
- [ ] Reference in MONTHLY_CALENDAR.md

---

## 💡 Teaching Notes

### Pedagogical Approach
- **Progressive Disclosure**: Start simple, add complexity gradually
- **Motivation First**: Explain why before how
- **Visual Learning**: Use diagrams and visualizations extensively
- **Active Learning**: Include exercises and experiments
- **Real-World Context**: Connect to practical applications
- **Performance Focus**: Always discuss time/space complexity

### Common Student Struggles
- Understanding why optimizations matter (address in Steps 2-4)
- Inverse Ackermann function (explain intuitively in Step 4)
- When to use Union-Find vs other approaches (address in Step 7)
- Implementation details (provide detailed comments)

### Success Criteria
Student completes tutorial when they can:
- [ ] Implement Union-Find from scratch
- [ ] Explain both optimizations clearly
- [ ] Analyze complexity correctly
- [ ] Recognize Union-Find problems
- [ ] Choose appropriate variant for use case
- [ ] Solve interview problems independently

---

## ✅ Definition of Done

Tutorial is complete when:
- [ ] All 7 steps implemented and tested
- [ ] All exercises created with solutions
- [ ] Documentation comprehensive and clear
- [ ] All examples compile and run
- [ ] Cross-references updated
- [ ] Zettelkasten integration complete
- [ ] Code reviewed for quality
- [ ] Student feedback incorporated (if available)
- [ ] Ready for independent learning

**Target Completion**: November 21, 2024

---

## Related Resources

- [[README]] - Mission 10 Tutorial overview and 7-step roadmap
- [[../../missions/Mission10/TODO]] - Main mission development TODO
- [[../../missions/Mission10/README]] - Union-Find V-Cycle requirements and design
- [[../../zettelkasten/Missions Overview]] - All missions and tutorials overview
- [[../../MONTHLY_CALENDAR]] - Week 6 schedule (Mission 10 focus: November 2-8)
- [[../../daily_study/rust_learning_week6_notes/Day37]] - Crate organization patterns
- [[examples/step1_basic_union_find]] - Basic implementation (completed)
- [[examples/step2_path_compression]] - Path compression optimization (completed)

*Tags: #mission10-tutorial #union-find #tutorial-roadmap #progressive-learning #todo #development-plan*
