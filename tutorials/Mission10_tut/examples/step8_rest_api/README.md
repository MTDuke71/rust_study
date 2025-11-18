# Step 8: REST API with OpenAPI/Swagger

This example demonstrates how to wrap the Union-Find data structure in a production-ready REST API using Axum and Utoipa.

## 🚀 Getting Started

1. **Run the server**:
   ```bash
   cargo run
   ```

2. **Access Documentation**:
   Open [http://localhost:8080/swagger-ui](http://localhost:8080/swagger-ui) in your browser to see the interactive API documentation.

3. **Quick Test**:
   ```bash
   # Create instance
   curl -X POST http://localhost:8080/api/v1/unionfind \
     -H "Content-Type: application/json" \
     -d '{"size": 10}'
   
   # Save the returned ID and test operations
   ID="<your-instance-id>"
   
   # Union elements
   curl -X POST "http://localhost:8080/api/v1/unionfind/$ID/union" \
     -H "Content-Type: application/json" \
     -d '{"element1": 3, "element2": 7}'
   
   # Check connectivity
   curl "http://localhost:8080/api/v1/unionfind/$ID/connected?element1=3&element2=7"
   
   # Get statistics
   curl "http://localhost:8080/api/v1/unionfind/$ID/stats"
   ```

## 📚 Complete Learning Guide

See [TUTORIAL.md](TUTORIAL.md) for:
- Detailed explanations of all concepts
- Step-by-step learning progression
- Comprehensive testing examples
- Common mistakes and solutions
- Performance considerations
- Practice exercises

## 🛠️ Project Structure

- `src/main.rs`: Entry point and server configuration
- `src/models.rs`: Request/Response structs with OpenAPI schemas
- `src/handlers.rs`: API endpoint implementations (6 endpoints)
- `src/state.rs`: Thread-safe application state management
- `src/openapi.rs`: OpenAPI specification definition
- `src/errors.rs`: Error types and HTTP error responses

## 📡 **API Endpoints**

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/unionfind` | Create new Union-Find instance |
| POST | `/api/v1/unionfind/{id}/union` | Union two elements |
| GET | `/api/v1/unionfind/{id}/find` | Find root of element |
| GET | `/api/v1/unionfind/{id}/connected` | Check if elements connected |
| GET | `/api/v1/unionfind/{id}/stats` | Get instance statistics |
| DELETE | `/api/v1/unionfind/{id}` | Delete instance |
| GET | `/health` | Health check |

## 🛠️ Technologies

- **Axum**: Ergonomic and modular web framework
- **Tokio**: Asynchronous runtime
- **Utoipa**: Compile-time OpenAPI generation
- **Serde**: Serialization/Deserialization
- **Tracing**: Structured logging

## ✅ Features

- ✅ Full CRUD operations for Union-Find instances
- ✅ Automatic OpenAPI documentation
- ✅ Interactive Swagger UI
- ✅ Input validation
- ✅ Proper error handling
- ✅ Thread-safe state management
- ✅ Production-ready code quality

---

*Next*: See [TUTORIAL.md](TUTORIAL.md) for comprehensive learning guide
*Mission 10*: See [../../../../../missions/Mission10/README.md](../../../../../missions/Mission10/README.md)

