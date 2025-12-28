# Union-Find REST API 🌐

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-ready REST API for Union-Find (Disjoint Set Union) data structures, built with Rust, Axum, and OpenAPI.

**Mission Context**: [[../../README|Mission 10 Tutorial]] | [[TUTORIAL|Comprehensive Tutorial Guide]] | [[../../../../../missions/Mission10/README|Mission 10]]

---

## Features ✨

- 🚀 **RESTful Design**: CRUD operations for Union-Find instances
- 📖 **OpenAPI Documentation**: Interactive Swagger UI at `/swagger-ui`
- 🔒 **Type-Safe**: Strongly-typed request/response models with validation
- ⚡ **High Performance**: O(α(n)) amortized operations (~16-21ns per op)
- 🧪 **Well-Tested**: Comprehensive unit + integration tests (16+ test cases)
- 🎯 **Production-Ready**: Structured error handling, logging, deployment guides
- 🐳 **Docker Support**: Multi-stage Dockerfile for containerized deployment
- 📊 **Benchmarked**: Performance baselines established with Criterion

---

## Quick Start 🚀

```bash
# Clone and run
git clone <repo-url>
cd tutorials/Mission10_tut/examples/step8_rest_api

# Run locally
cargo run --release

# Server starts on http://localhost:8080
# Swagger UI: http://localhost:8080/swagger-ui
# OpenAPI spec: http://localhost:8080/api-docs/openapi.json
```

### Docker Deployment

```bash
# Build and run with Docker Compose
docker compose up -d

# Check logs
docker logs unionfind-rest-api

# Stop
docker compose down
```

See [DOCKER_DEPLOYMENT.md](DOCKER_DEPLOYMENT.md) for detailed deployment instructions.

---

## API Examples 📡

### Create Instance
```bash
POST /api/v1/unionfind
{
  "size": 10
}
# → {"id": "550e8400-...", "size": 10}
```

### Union Elements
```bash
POST /api/v1/unionfind/{id}/union
{
  "element1": 0,
  "element2": 5
}
# → {"merged": true, "root": 0}
```

### Check Connected
```bash
GET /api/v1/unionfind/{id}/connected?element1=0&element2=5
# → {"connected": true}
```

### Get Statistics
```bash
GET /api/v1/unionfind/{id}/stats
# → {"total_elements": 10, "num_components": 5}
```

### Complete Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/unionfind` | Create new Union-Find instance |
| POST | `/api/v1/unionfind/{id}/union` | Union two elements |
| GET | `/api/v1/unionfind/{id}/find` | Find root of element |
| GET | `/api/v1/unionfind/{id}/connected` | Check if elements connected |
| GET | `/api/v1/unionfind/{id}/stats` | Get instance statistics |
| DELETE | `/api/v1/unionfind/{id}` | Delete instance |
| GET | `/health` | Health check |

---

## Architecture 🏗️

```
HTTP Request → Axum Router → Handlers → AppState (DashMap) → UnionFind
```

**Key Components**:
- `main.rs` / `lib.rs`: Server setup, routing, middleware
- `handlers.rs`: 6 endpoint implementations
- `state.rs`: Thread-safe storage (Arc<DashMap>)
- `models.rs`: OpenAPI-annotated request/response types

---

## Documentation 📚

- **[Swagger UI](http://localhost:8080/swagger-ui)**: Interactive API docs
- **[TUTORIAL.md](TUTORIAL.md)**: Comprehensive learning guide (1400+ lines)
- **[PERFORMANCE.md](PERFORMANCE.md)**: Benchmark results and analysis
- **[DOCKER_DEPLOYMENT.md](DOCKER_DEPLOYMENT.md)**: Deployment guide
- **[DAY13_IMPLEMENTATION_GUIDE.md](DAY13_IMPLEMENTATION_GUIDE.md)**: Error handling deep dive

---

## Testing 🧪

```bash
# Run all tests
cargo test

# Integration tests only
cargo test --test api_tests

# Run benchmarks
cargo bench

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

**Test Coverage**:
- 16+ integration tests (full API validation)
- Error handling tests (all error codes)
- Performance benchmarks (Criterion)

---

## Performance 📊

| Operation | Time (avg) | Complexity |
|-----------|-----------|------------|
| Create (100 elements) | 344 ns | O(n) |
| Union | 20.47 ns | O(α(n)) |
| Find | 15.96 ns | O(α(n)) |
| Connected | 16.09 ns | O(α(n)) |

**α(n)** = Inverse Ackermann (≈ constant)

See [PERFORMANCE.md](PERFORMANCE.md) for detailed analysis.

---

## Technologies 🛠️

- **[Axum 0.7](https://docs.rs/axum)**: Web framework
- **[Tokio](https://tokio.rs/)**: Async runtime
- **[Utoipa 4.x](https://docs.rs/utoipa)**: OpenAPI generation
- **[DashMap](https://docs.rs/dashmap)**: Concurrent hashmap
- **[Tracing](https://docs.rs/tracing)**: Structured logging

---

## Error Handling 🔒

Structured errors with semantic codes:

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

**Error Codes**: `INVALID_SIZE`, `INSTANCE_NOT_FOUND`, `ELEMENT_OUT_OF_BOUNDS`

---

## Links

**Tutorial Progression**:
- [[../../README]] - Mission 10 Tutorial overview (7 steps)
- [[TUTORIAL]] - Comprehensive REST API learning guide
- [[../step5_applications]] - Step 5: Real-world applications

**Mission Context**:
- [[../../../../../missions/Mission10/README]] - Mission 10 implementation
- [[../../../../../missions/Mission10/PHASE6_REVIEW]] - Phase 6 review
- [[../../../../../zettelkasten/missions/mission-10]] - Zettelkasten

**Learning Integration**:
- [[../../../../../zettelkasten/Missions Overview]] - All missions progress
- [[../../../../../zettelkasten/Quality Assurance]] - Quality standards

---

**Status**: ✅ Production-Ready | **Version**: 0.1.0 | **Updated**: 2025-12-28

