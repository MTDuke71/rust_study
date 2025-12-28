# Multi-stage Dockerfile for Union-Find REST API
# Optimized for small image size and fast builds

# Stage 1: Build the application
FROM rust:latest as builder

WORKDIR /app

# Copy workspace dependencies (Mission10)
COPY missions/Mission10 /missions/Mission10

# Copy tutorial source files
COPY tutorials/Mission10_tut/examples/step8_rest_api/Cargo.toml tutorials/Mission10_tut/examples/step8_rest_api/Cargo.lock ./
COPY tutorials/Mission10_tut/examples/step8_rest_api/src ./src
COPY tutorials/Mission10_tut/examples/step8_rest_api/benches ./benches
COPY tutorials/Mission10_tut/examples/step8_rest_api/tests ./tests

# Build for release
RUN cargo build --release --bin step8_rest_api

# Stage 2: Runtime image
FROM debian:bookworm-slim

# Install CA certificates for HTTPS (if needed in future)
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 apiuser

# Copy the binary from builder
COPY --from=builder /app/target/release/step8_rest_api /usr/local/bin/step8_rest_api

# Set ownership
RUN chown apiuser:apiuser /usr/local/bin/step8_rest_api

# Switch to non-root user
USER apiuser

# Expose port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["step8_rest_api"]
