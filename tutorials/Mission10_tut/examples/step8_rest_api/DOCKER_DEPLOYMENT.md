# Docker Deployment Guide 🐳

## Quick Start

### Build and Run with Docker Compose (Recommended)

```bash
# Build and start the container
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the container
docker-compose down
```

The API will be available at:
- **API Endpoints**: http://localhost:8080/api/v1
- **Swagger UI**: http://localhost:8080/swagger-ui
- **Health Check**: http://localhost:8080/health

---

## Manual Docker Commands

### Build the Image

```bash
# Build the Docker image
docker build -t unionfind-api:latest .

# Build with no cache (clean build)
docker build --no-cache -t unionfind-api:latest .
```

### Run the Container

```bash
# Run in foreground
docker run -p 8080:8080 unionfind-api:latest

# Run in background (detached)
docker run -d -p 8080:8080 --name unionfind-api unionfind-api:latest

# Run with custom log level
docker run -d -p 8080:8080 -e RUST_LOG=debug --name unionfind-api unionfind-api:latest
```

### Manage the Container

```bash
# View logs
docker logs unionfind-api
docker logs -f unionfind-api  # Follow logs

# Stop the container
docker stop unionfind-api

# Start the container
docker start unionfind-api

# Remove the container
docker rm unionfind-api

# Remove the image
docker rmi unionfind-api:latest
```

---

## Docker Image Details

### Multi-Stage Build

The Dockerfile uses a **multi-stage build** to minimize image size:

1. **Builder Stage** (rust:1.75)
   - Full Rust toolchain
   - Compiles the application
   - ~2GB image size

2. **Runtime Stage** (debian:bookworm-slim)
   - Only the compiled binary
   - Minimal dependencies
   - ~80MB final image size

### Security Features

- ✅ **Non-root user** - Runs as `apiuser` (UID 1000)
- ✅ **Minimal base image** - Debian slim reduces attack surface
- ✅ **CA certificates** - Included for HTTPS support
- ✅ **Health check** - Automatic container health monitoring

---

## Testing the Docker Container

### 1. Verify the Container is Running

```bash
docker ps
```

Expected output:
```
CONTAINER ID   IMAGE                    STATUS         PORTS                    NAMES
abc123def456   unionfind-api:latest    Up 2 minutes   0.0.0.0:8080->8080/tcp   unionfind-api
```

### 2. Check Health Status

```bash
docker inspect --format='{{.State.Health.Status}}' unionfind-api
```

Should return: `healthy`

### 3. Test API Endpoints

```bash
# Health check
curl http://localhost:8080/health

# Create instance
curl -X POST http://localhost:8080/api/v1/unionfind \
  -H "Content-Type: application/json" \
  -d '{"size": 10}'

# Visit Swagger UI
# Open browser: http://localhost:8080/swagger-ui
```

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |
| `SERVER_HOST` | `0.0.0.0` | Bind address (use 0.0.0.0 for Docker) |
| `SERVER_PORT` | `8080` | Port to listen on |

### Custom Configuration

```bash
# Run with debug logging
docker run -d -p 8080:8080 \
  -e RUST_LOG=debug \
  --name unionfind-api \
  unionfind-api:latest

# Run on different port
docker run -d -p 9090:8080 \
  --name unionfind-api \
  unionfind-api:latest
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs unionfind-api

# Common issues:
# - Port 8080 already in use → Change port mapping
# - Build failed → Check Cargo.toml and dependencies
```

### Can't Connect to API

```bash
# Verify container is running
docker ps

# Check port mapping
docker port unionfind-api

# Test from inside container
docker exec unionfind-api curl http://localhost:8080/health
```

### High Memory Usage

```bash
# Set memory limit
docker run -d -p 8080:8080 \
  --memory="512m" \
  --name unionfind-api \
  unionfind-api:latest
```

### Rebuild After Code Changes

```bash
# Stop and remove old container
docker-compose down

# Rebuild and start
docker-compose up -d --build

# Or manually:
docker build -t unionfind-api:latest .
docker run -d -p 8080:8080 --name unionfind-api unionfind-api:latest
```

---

## Performance Optimization

### Build Cache

Docker caches layers. To optimize build times:

```dockerfile
# In Dockerfile, copy dependencies first
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch  # Cache dependencies

# Then copy source
COPY src ./src
```

### Image Size

Current setup produces:
- **Builder image**: ~2GB (not kept)
- **Runtime image**: ~80MB (deployed)

### Resource Limits

```yaml
# In docker-compose.yml
services:
  unionfind-api:
    # ... other config ...
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

---

## Production Considerations

### Things NOT Included (Future Enhancements)

- ❌ Persistent storage (currently in-memory only)
- ❌ Redis/database integration
- ❌ TLS/HTTPS (use reverse proxy)
- ❌ Rate limiting
- ❌ Authentication

### Recommended Production Setup

```bash
# Use docker-compose with reverse proxy
version: '3.8'
services:
  unionfind-api:
    # ... API config ...
  
  nginx:
    image: nginx:alpine
    ports:
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/ssl
    depends_on:
      - unionfind-api
```

---

## Docker Commands Quick Reference

```bash
# BUILD
docker build -t unionfind-api .
docker-compose build

# RUN
docker run -d -p 8080:8080 unionfind-api
docker-compose up -d

# LOGS
docker logs -f unionfind-api
docker-compose logs -f

# STOP
docker stop unionfind-api
docker-compose down

# CLEAN UP
docker system prune -a    # Remove all unused images
docker volume prune       # Remove unused volumes
```

---

## Success Checklist ✅

After deployment, verify:

- [ ] Container starts successfully
- [ ] Health check returns "healthy"
- [ ] API responds to requests
- [ ] Swagger UI loads at /swagger-ui
- [ ] Logs show no errors
- [ ] Can create/query Union-Find instances
- [ ] Container restarts automatically (if using docker-compose)

---

**Docker Status**: ✅ Ready for local deployment and testing!
