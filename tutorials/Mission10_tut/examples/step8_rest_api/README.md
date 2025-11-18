# Step 8: REST API with OpenAPI/Swagger

This example demonstrates how to wrap the Union-Find data structure in a production-ready REST API using Axum and Utoipa.

## 🚀 Getting Started

1. **Run the server**:
   ```bash
   cargo run
   ```

2. **Access Documentation**:
   Open [http://localhost:8080/swagger-ui](http://localhost:8080/swagger-ui) in your browser to see the interactive API documentation.

3. **Test Endpoints**:
   You can use `curl` or the Swagger UI to interact with the API.

   ```bash
   # Check health
   curl http://localhost:8080/health
   ```

## 📚 Project Structure

- `src/main.rs`: Entry point and server configuration
- `src/models.rs`: Request/Response structs with OpenAPI schemas
- `src/handlers.rs`: API endpoint implementations
- `src/state.rs`: Application state management
- `src/openapi.rs`: OpenAPI specification definition

## 🛠️ Technologies

- **Axum**: Ergonomic and modular web framework
- **Tokio**: Asynchronous runtime
- **Utoipa**: Compile-time OpenAPI generation
- **Serde**: Serialization/Deserialization
- **Tracing**: Structured logging
