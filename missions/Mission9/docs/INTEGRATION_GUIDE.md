# Mission 9 Integration Guide

## Overview

This guide shows how to integrate Mission9 pathfinding into your Rust projects, Python applications, web services, and other systems.

## Rust Integration

### Adding as Dependency

**Option 1: Local path dependency**
```toml
[dependencies]
mission9 = { path = "../path/to/missions/Mission9" }
```

**Option 2: Git dependency** (if published to repository)
```toml
[dependencies]
mission9 = { git = "https://github.com/yourusername/rust_study", tag = "mission9-v1.0" }
```

### Basic Usage

```rust
use mission9::*;

fn main() -> Result<(), PathfindingError> {
    // Create graph
    let mut graph = SimpleWeightedGraph::new(5);
    graph.add_edge(0, 1, 1.0)?;
    graph.add_edge(1, 2, 1.0)?;
    graph.add_edge(2, 3, 1.5)?;
    graph.add_edge(3, 4, 1.0)?;
    
    // Find path
    let pathfinder = DijkstraPathfinder::new();
    let result = pathfinder.find_path(&graph, 0, 4)?;
    
    println!("Path: {:?}", result.path);
    println!("Cost: {}", result.cost);
    
    Ok(())
}
```

### Web Service Integration (Actix-web)

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mission9::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
struct PathQuery {
    start: u32,
    goal: u32,
    algorithm: String,
}

#[derive(Serialize)]
struct PathResponse {
    path: Vec<u32>,
    cost: f64,
    nodes_explored: usize,
    search_time_ms: f64,
}

async fn find_path(
    query: web::Json<PathQuery>,
    graph: web::Data<Arc<SimpleWeightedGraph>>,
) -> impl Responder {
    let result = match query.algorithm.as_str() {
        "dijkstra" => {
            let pathfinder = DijkstraPathfinder::new();
            pathfinder.find_path(graph.get_ref(), query.start, query.goal)
        }
        "astar" => {
            let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
            pathfinder.find_path(graph.get_ref(), query.start, query.goal)
        }
        _ => return HttpResponse::BadRequest().body("Invalid algorithm"),
    };
    
    match result {
        Ok(path_result) => HttpResponse::Ok().json(PathResponse {
            path: path_result.path,
            cost: path_result.cost,
            nodes_explored: path_result.nodes_explored,
            search_time_ms: path_result.search_time.as_secs_f64() * 1000.0,
        }),
        Err(e) => HttpResponse::NotFound().body(format!("No path found: {:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load graph once at startup
    let graph = /* load from file */;
    let graph_data = web::Data::new(Arc::new(graph));
    
    HttpServer::new(move || {
        App::new()
            .app_data(graph_data.clone())
            .route("/find-path", web::post().to(find_path))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Game Engine Integration (Bevy)

```rust
use bevy::prelude::*;
use mission9::*;

#[derive(Resource)]
struct NavGraph {
    graph: SimpleWeightedGraph,
    context: HeuristicContext,
}

#[derive(Component)]
struct PathfindingAgent {
    current_pos: u32,
    target_pos: u32,
    path: Vec<u32>,
    path_index: usize,
}

fn setup_navigation(mut commands: Commands) {
    // Create navigation graph from game world
    let mut graph = SimpleWeightedGraph::new(1000);
    let mut context = HeuristicContext::new();
    
    // Build graph from game level data...
    
    commands.insert_resource(NavGraph { graph, context });
}

fn pathfinding_system(
    nav_graph: Res<NavGraph>,
    mut agents: Query<&mut PathfindingAgent>,
) {
    for mut agent in agents.iter_mut() {
        if agent.path.is_empty() {
            // Calculate new path
            let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
            if let Ok(result) = pathfinder.find_path_with_context(
                &nav_graph.graph,
                agent.current_pos,
                agent.target_pos,
                &nav_graph.context,
            ) {
                agent.path = result.path;
                agent.path_index = 0;
            }
        }
    }
}

fn movement_system(
    time: Res<Time>,
    mut agents: Query<(&mut PathfindingAgent, &mut Transform)>,
) {
    for (mut agent, mut transform) in agents.iter_mut() {
        if agent.path_index < agent.path.len() {
            // Move towards next waypoint
            let next_waypoint = agent.path[agent.path_index];
            // ... movement logic ...
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_navigation)
        .add_systems(Update, (pathfinding_system, movement_system))
        .run();
}
```

## Python Integration (PyO3)

### Creating Python Bindings

```rust
// src/python_bindings.rs
use pyo3::prelude::*;
use mission9::*;

#[pyclass]
struct PyGraph {
    graph: SimpleWeightedGraph,
}

#[pymethods]
impl PyGraph {
    #[new]
    fn new(node_count: usize) -> Self {
        PyGraph {
            graph: SimpleWeightedGraph::new(node_count),
        }
    }
    
    fn add_edge(&mut self, from: u32, to: u32, weight: f64) -> PyResult<()> {
        self.graph.add_edge(from, to, weight)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))
    }
    
    fn find_path(&self, start: u32, goal: u32) -> PyResult<(Vec<u32>, f64)> {
        let pathfinder = DijkstraPathfinder::new();
        match pathfinder.find_path(&self.graph, start, goal) {
            Ok(result) => Ok((result.path, result.cost)),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e))),
        }
    }
}

#[pymodule]
fn mission9_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    Ok(())
}
```

**Cargo.toml for Python bindings:**
```toml
[lib]
name = "mission9_py"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
mission9 = { path = ".." }
```

**Build and use:**
```bash
# Build Python module
maturin develop

# Use in Python
python3 << EOF
import mission9_py

graph = mission9_py.PyGraph(5)
graph.add_edge(0, 1, 1.0)
graph.add_edge(1, 2, 1.0)
graph.add_edge(2, 3, 1.5)

path, cost = graph.find_path(0, 3)
print(f"Path: {path}, Cost: {cost}")
EOF
```

### Python via CLI (Subprocess)

```python
import subprocess
import json
from typing import List, Tuple, Optional

class Mission9Pathfinder:
    def __init__(self, mission9_bin: str = "mission9"):
        self.bin = mission9_bin
    
    def generate_graph(self, graph_type: str, nodes: int, output: str):
        """Generate a test graph."""
        subprocess.run([
            self.bin, "generate",
            "--graph-type", graph_type,
            "--nodes", str(nodes),
            "--file", output
        ], check=True)
    
    def find_path(
        self,
        graph_file: str,
        start: int,
        goal: int,
        algorithm: str = "astar"
    ) -> Optional[dict]:
        """Find path and return result as dict."""
        result = subprocess.run([
            self.bin, "--output", "json", "find-path",
            "--graph", graph_file,
            "--start", str(start),
            "--goal", str(goal),
            "--algorithm", algorithm
        ], capture_output=True, text=True, check=True)
        
        return json.loads(result.stdout)
    
    def batch_process(
        self,
        graph_file: str,
        queries_csv: str,
        output_csv: str,
        algorithm: str = "astar"
    ):
        """Process batch of queries."""
        subprocess.run([
            self.bin, "batch",
            "--graph", graph_file,
            "--queries", queries_csv,
            "--output", output_csv,
            "--algorithm", algorithm
        ], check=True)

# Usage example
pathfinder = Mission9Pathfinder()
pathfinder.generate_graph("grid", 100, "test.json")
result = pathfinder.find_path("test.json", 0, 99)
print(f"Path cost: {result['cost']}")
print(f"Path length: {len(result['path'])}")
```

## JavaScript/TypeScript Integration (WASM)

### Building WASM Module

```rust
// src/wasm_bindings.rs
use wasm_bindgen::prelude::*;
use mission9::*;

#[wasm_bindgen]
pub struct WasmGraph {
    graph: SimpleWeightedGraph,
}

#[wasm_bindgen]
impl WasmGraph {
    #[wasm_bindgen(constructor)]
    pub fn new(node_count: usize) -> WasmGraph {
        WasmGraph {
            graph: SimpleWeightedGraph::new(node_count),
        }
    }
    
    #[wasm_bindgen]
    pub fn add_edge(&mut self, from: u32, to: u32, weight: f64) -> Result<(), JsValue> {
        self.graph.add_edge(from, to, weight)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }
    
    #[wasm_bindgen]
    pub fn find_path(&self, start: u32, goal: u32) -> Result<JsValue, JsValue> {
        let pathfinder = DijkstraPathfinder::new();
        match pathfinder.find_path(&self.graph, start, goal) {
            Ok(result) => {
                let js_result = js_sys::Object::new();
                js_sys::Reflect::set(
                    &js_result,
                    &"path".into(),
                    &serde_wasm_bindgen::to_value(&result.path)?
                )?;
                js_sys::Reflect::set(
                    &js_result,
                    &"cost".into(),
                    &JsValue::from_f64(result.cost)
                )?;
                Ok(js_result.into())
            }
            Err(e) => Err(JsValue::from_str(&format!("{:?}", e))),
        }
    }
}
```

**TypeScript usage:**
```typescript
import init, { WasmGraph } from './pkg/mission9_wasm';

async function main() {
    await init();
    
    const graph = new WasmGraph(5);
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.5);
    
    const result = graph.find_path(0, 3);
    console.log(`Path: ${result.path}, Cost: ${result.cost}`);
}

main();
```

## C/C++ Integration (FFI)

### Creating C API

```rust
// src/ffi.rs
use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use mission9::*;

#[repr(C)]
pub struct CPathResult {
    path: *mut u32,
    path_len: usize,
    cost: f64,
}

#[no_mangle]
pub extern "C" fn mission9_create_graph(node_count: usize) -> *mut SimpleWeightedGraph {
    Box::into_raw(Box::new(SimpleWeightedGraph::new(node_count)))
}

#[no_mangle]
pub extern "C" fn mission9_add_edge(
    graph: *mut SimpleWeightedGraph,
    from: u32,
    to: u32,
    weight: f64,
) -> i32 {
    if graph.is_null() {
        return -1;
    }
    unsafe {
        match (*graph).add_edge(from, to, weight) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn mission9_find_path(
    graph: *const SimpleWeightedGraph,
    start: u32,
    goal: u32,
) -> CPathResult {
    if graph.is_null() {
        return CPathResult {
            path: std::ptr::null_mut(),
            path_len: 0,
            cost: -1.0,
        };
    }
    
    unsafe {
        let pathfinder = DijkstraPathfinder::new();
        match pathfinder.find_path(&*graph, start, goal) {
            Ok(result) => {
                let mut path_vec = result.path.into_boxed_slice();
                let path_ptr = path_vec.as_mut_ptr();
                let path_len = path_vec.len();
                std::mem::forget(path_vec);
                
                CPathResult {
                    path: path_ptr,
                    path_len,
                    cost: result.cost,
                }
            }
            Err(_) => CPathResult {
                path: std::ptr::null_mut(),
                path_len: 0,
                cost: -1.0,
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn mission9_free_path(path: *mut u32, len: usize) {
    if !path.is_null() {
        unsafe {
            Vec::from_raw_parts(path, len, len);
        }
    }
}

#[no_mangle]
pub extern "C" fn mission9_free_graph(graph: *mut SimpleWeightedGraph) {
    if !graph.is_null() {
        unsafe {
            Box::from_raw(graph);
        }
    }
}
```

**C header file (mission9.h):**
```c
#ifndef MISSION9_H
#define MISSION9_H

#include <stdint.h>
#include <stddef.h>

typedef struct {
    uint32_t *path;
    size_t path_len;
    double cost;
} CPathResult;

void *mission9_create_graph(size_t node_count);
int mission9_add_edge(void *graph, uint32_t from, uint32_t to, double weight);
CPathResult mission9_find_path(const void *graph, uint32_t start, uint32_t goal);
void mission9_free_path(uint32_t *path, size_t len);
void mission9_free_graph(void *graph);

#endif
```

**C usage example:**
```c
#include "mission9.h"
#include <stdio.h>

int main() {
    void *graph = mission9_create_graph(5);
    
    mission9_add_edge(graph, 0, 1, 1.0);
    mission9_add_edge(graph, 1, 2, 1.0);
    mission9_add_edge(graph, 2, 3, 1.5);
    
    CPathResult result = mission9_find_path(graph, 0, 3);
    
    if (result.path != NULL) {
        printf("Path cost: %f\n", result.cost);
        printf("Path: ");
        for (size_t i = 0; i < result.path_len; i++) {
            printf("%u ", result.path[i]);
        }
        printf("\n");
        
        mission9_free_path(result.path, result.path_len);
    }
    
    mission9_free_graph(graph);
    return 0;
}
```

## REST API Service

### Using Rocket Framework

```rust
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use mission9::*;
use std::sync::Arc;

#[derive(Deserialize)]
struct PathRequest {
    start: u32,
    goal: u32,
    algorithm: Option<String>,
}

#[derive(Serialize)]
struct PathResponse {
    success: bool,
    path: Option<Vec<u32>>,
    cost: Option<f64>,
    nodes_explored: Option<usize>,
    error: Option<String>,
}

#[post("/find-path", data = "<request>")]
fn find_path(
    request: Json<PathRequest>,
    graph: &State<Arc<SimpleWeightedGraph>>,
) -> Json<PathResponse> {
    let algorithm = request.algorithm.as_deref().unwrap_or("dijkstra");
    
    let result = match algorithm {
        "dijkstra" => {
            let pathfinder = DijkstraPathfinder::new();
            pathfinder.find_path(graph.inner(), request.start, request.goal)
        }
        "astar" => {
            let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
            pathfinder.find_path(graph.inner(), request.start, request.goal)
        }
        _ => {
            return Json(PathResponse {
                success: false,
                path: None,
                cost: None,
                nodes_explored: None,
                error: Some("Invalid algorithm".to_string()),
            });
        }
    };
    
    match result {
        Ok(path_result) => Json(PathResponse {
            success: true,
            path: Some(path_result.path),
            cost: Some(path_result.cost),
            nodes_explored: Some(path_result.nodes_explored),
            error: None,
        }),
        Err(e) => Json(PathResponse {
            success: false,
            path: None,
            cost: None,
            nodes_explored: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[launch]
fn rocket() -> _ {
    // Load graph at startup
    let graph = /* load from file */;
    
    rocket::build()
        .manage(Arc::new(graph))
        .mount("/api", routes![find_path])
}
```

## Database Integration

### Storing Graphs in PostgreSQL

```rust
use sqlx::postgres::PgPool;
use mission9::*;

async fn save_graph_to_db(
    pool: &PgPool,
    graph: &SimpleWeightedGraph,
    graph_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Store graph metadata
    sqlx::query!(
        "INSERT INTO graphs (name, node_count) VALUES ($1, $2)",
        graph_name,
        graph.node_count() as i32
    )
    .execute(pool)
    .await?;
    
    // Store edges
    for node in 0..graph.node_count() as u32 {
        for (neighbor, weight) in graph.neighbors(node) {
            sqlx::query!(
                "INSERT INTO edges (graph_name, from_node, to_node, weight) VALUES ($1, $2, $3, $4)",
                graph_name,
                node as i32,
                neighbor as i32,
                weight
            )
            .execute(pool)
            .await?;
        }
    }
    
    Ok(())
}

async fn load_graph_from_db(
    pool: &PgPool,
    graph_name: &str,
) -> Result<SimpleWeightedGraph, Box<dyn std::error::Error>> {
    // Get node count
    let graph_meta = sqlx::query!(
        "SELECT node_count FROM graphs WHERE name = $1",
        graph_name
    )
    .fetch_one(pool)
    .await?;
    
    let mut graph = SimpleWeightedGraph::new(graph_meta.node_count as usize);
    
    // Load edges
    let edges = sqlx::query!(
        "SELECT from_node, to_node, weight FROM edges WHERE graph_name = $1",
        graph_name
    )
    .fetch_all(pool)
    .await?;
    
    for edge in edges {
        graph.add_edge(
            edge.from_node as u32,
            edge.to_node as u32,
            edge.weight,
        )?;
    }
    
    Ok(graph)
}
```

## Performance Considerations

### Connection Pooling
```rust
// Reuse pathfinder instances
lazy_static! {
    static ref DIJKSTRA: DijkstraPathfinder = DijkstraPathfinder::new();
    static ref ASTAR: AstarPathfinder<EuclideanHeuristic> = 
        AstarPathfinder::new(EuclideanHeuristic);
}
```

### Caching
```rust
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref PATH_CACHE: RwLock<HashMap<(u32, u32), PathResult>> = 
        RwLock::new(HashMap::new());
}

fn find_path_cached(
    graph: &SimpleWeightedGraph,
    start: u32,
    goal: u32,
) -> Result<PathResult, PathfindingError> {
    // Check cache
    {
        let cache = PATH_CACHE.read().unwrap();
        if let Some(result) = cache.get(&(start, goal)) {
            return Ok(result.clone());
        }
    }
    
    // Compute
    let pathfinder = DijkstraPathfinder::new();
    let result = pathfinder.find_path(graph, start, goal)?;
    
    // Store in cache
    {
        let mut cache = PATH_CACHE.write().unwrap();
        cache.insert((start, goal), result.clone());
    }
    
    Ok(result)
}
```

## Testing Integrations

### Integration Test Template

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_basic_integration() {
        let mut graph = SimpleWeightedGraph::new(5);
        graph.add_edge(0, 1, 1.0).unwrap();
        graph.add_edge(1, 2, 1.0).unwrap();
        
        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 0, 2).unwrap();
        
        assert_eq!(result.path, vec![0, 1, 2]);
        assert_eq!(result.cost, 2.0);
    }
    
    #[tokio::test]
    async fn test_async_integration() {
        // Test async/await integration
        let graph = /* ... */;
        let result = tokio::task::spawn_blocking(move || {
            let pathfinder = DijkstraPathfinder::new();
            pathfinder.find_path(&graph, 0, 10)
        }).await.unwrap();
        
        assert!(result.is_ok());
    }
}
```

## Deployment

### Docker Container

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/mission9 /usr/local/bin/
COPY --from=builder /app/target/release/mission9-grid /usr/local/bin/
CMD ["mission9"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mission9-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: mission9
  template:
    metadata:
      labels:
        app: mission9
    spec:
      containers:
      - name: mission9
        image: mission9:latest
        ports:
        - containerPort: 8080
        resources:
          limits:
            memory: "512Mi"
            cpu: "500m"
```

## See Also

- [API Documentation](API_DOCUMENTATION.md) - Complete library reference
- [CLI Guide](CLI_GUIDE.md) - Command-line usage
- [Performance Tuning Guide](PERFORMANCE_TUNING.md) - Optimization strategies
