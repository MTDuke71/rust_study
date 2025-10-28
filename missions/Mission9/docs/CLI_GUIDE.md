# Mission 9 CLI Guide

## Overview

Mission 9 provides two production-ready command-line interfaces for pathfinding:

1. **`mission9`** - Graph-based pathfinding (Dijkstra, A*)
2. **`mission9-grid`** - Grid-based pathfinding (Bidirectional algorithms)

## Installation

Build both CLIs:
```bash
cargo build --release
```

The binaries will be in `target/release/`:
- `mission9.exe` (or `mission9` on Unix)
- `mission9-grid.exe` (or `mission9-grid` on Unix)

## CLI 1: Graph-Based Pathfinding (`mission9`)

### Commands

#### Generate Graph
Create test graphs in various formats:

```bash
# Generate a 10x10 grid
mission9 generate --graph-type grid --nodes 100 --file graph.json

# Generate a random graph
mission9 generate --graph-type random --nodes 500 --file random.json

# Generate a tree
mission9 generate --graph-type tree --nodes 200 --file tree.json --format csv

# Generate a complete graph
mission9 generate --graph-type complete --nodes 50 --file complete.json
```

**Options:**
- `--graph-type`: `grid`, `random`, `tree`, `complete`
- `--nodes`: Number of nodes (default: 100)
- `--file`: Output file path
- `--format`: `json` or `csv` (default: json)

#### Find Path
Find optimal path between two nodes:

```bash
# Basic pathfinding with A*
mission9 find-path --graph graph.json --start 0 --goal 99 --algorithm astar

# Use Dijkstra instead
mission9 find-path --graph graph.json --start 0 --goal 99 --algorithm dijkstra

# With verbose output
mission9 --verbose find-path --graph graph.json --start 0 --goal 99 --algorithm astar

# Generate visualization
mission9 find-path --graph graph.json --start 0 --goal 99 --algorithm astar --visualize path.dot

# JSON output format
mission9 --output json find-path --graph graph.json --start 0 --goal 99 --algorithm astar
```

**Options:**
- `--graph`: Input graph file (JSON or CSV)
- `--start`: Start node ID
- `--goal`: Goal node ID
- `--algorithm`: `dijkstra` or `astar`
- `--heuristic`: For A*, choose `manhattan`, `euclidean`, `chebyshev`, or `zero`
- `--visualize`: Output path visualization to DOT file

#### Batch Processing
Process multiple queries from CSV:

```bash
# Create queries file (queries.csv):
# start,goal
# 0,99
# 10,50
# 25,75

# Run batch processing
mission9 batch --graph graph.json --queries queries.csv --output results.csv --algorithm astar

# View results
cat results.csv
```

**Output CSV format:**
```csv
start,goal,cost,path_length,nodes_explored,time_ms
0,99,18.00,19,100,0.00
10,50,12.00,13,75,0.00
```

#### Graph Information
Display graph statistics:

```bash
mission9 info --graph graph.json
```

**Output:**
- Node count and edge count
- Degree statistics (min, max, average)
- Coordinate availability (for A* heuristics)

#### Benchmark
Compare algorithm performance:

```bash
mission9 benchmark --graph graph.json --start 0 --goal 99 --iterations 1000
```

**Output:**
- Total time and average time per query
- Speedup comparison (A* vs Dijkstra)

### File Formats

#### JSON Format
```json
{
  "nodes": 100,
  "edges": [
    {"from": 0, "to": 1, "weight": 1.0},
    {"from": 1, "to": 2, "weight": 1.0}
  ],
  "coordinates": {
    "0": {"x": 0.0, "y": 0.0},
    "1": {"x": 1.0, "y": 0.0}
  }
}
```

#### CSV Format
```csv
from,to,weight
0,1,1.0
1,2,1.0
2,3,1.5
```

### Visualization

Generate DOT files for Graphviz:

```bash
# Generate visualization
mission9 find-path --graph graph.json --start 0 --goal 99 --visualize path.dot

# Render with Graphviz
dot -Tpng path.dot -o path.png

# Or use other formats
dot -Tsvg path.dot -o path.svg
dot -Tpdf path.dot -o path.pdf
```

## CLI 2: Grid-Based Pathfinding (`mission9-grid`)

### Commands

#### Find Path in Grid
Navigate through coordinate-based grids:

```bash
# Basic 20x20 grid pathfinding
mission9-grid find-path --width 20 --height 20 --start 0,0 --goal 19,19

# With obstacles (vertical wall at x=10)
mission9-grid find-path --width 20 --height 20 --start 0,0 --goal 19,19 \
  --obstacles 10,0 10,1 10,2 10,3 10,4 10,5 10,6 10,7 10,8 10,9

# Use bidirectional Dijkstra
mission9-grid find-path --width 20 --height 20 --start 0,0 --goal 19,19 \
  --algorithm bidirectional-dijkstra

# Verbose output with path coordinates
mission9-grid --verbose find-path --width 20 --height 20 --start 0,0 --goal 19,19
```

**Options:**
- `--width`: Grid width
- `--height`: Grid height
- `--start`: Start coordinate (format: `x,y`)
- `--goal`: Goal coordinate (format: `x,y`)
- `--algorithm`: `bidirectional-dijkstra` or `bidirectional-astar`
- `--obstacles`: Space-separated obstacle coordinates

#### Benchmark Grid Algorithms
Compare bidirectional algorithm performance:

```bash
# Benchmark on 50x50 grid
mission9-grid benchmark --width 50 --height 50 --iterations 500

# Larger grid
mission9-grid --verbose benchmark --width 100 --height 100 --iterations 100
```

### Grid Visualization

The CLI automatically displays ASCII visualization for grids ≤50x50:

```
📊 Grid visualization:
  S • • • • █ · · · ·
  · · · · • █ · · · ·
  · · · · • █ · · · ·
  · · · · • • • • • •
  · · · · · · · · · G

  Legend: S=Start, G=Goal, •=Path, █=Obstacle, ·=Empty
```

## Real-World Examples

### Example 1: Route Planning
```bash
# Generate city road network (grid approximation)
mission9 generate --graph-type grid --nodes 400 --file city.json

# Find route from location A to B
mission9 find-path --graph city.json --start 0 --goal 399 --algorithm astar --visualize route.dot

# Render map
dot -Tpng route.dot -o route_map.png
```

### Example 2: Warehouse Navigation
```bash
# 30x30 warehouse with shelf obstacles
mission9-grid find-path --width 30 --height 30 --start 0,0 --goal 29,29 \
  --obstacles 10,0 10,1 10,2 10,3 10,4 10,5 20,0 20,1 20,2 20,3 20,4 20,5 \
  --algorithm bidirectional-astar
```

### Example 3: Performance Analysis
```bash
# Generate test graph
mission9 generate --graph-type random --nodes 1000 --file perf_test.json

# Benchmark different sizes
for size in 100 500 1000 5000; do
  mission9 generate --graph-type random --nodes $size --file "test_${size}.json"
  mission9 benchmark --graph "test_${size}.json" --start 0 --goal $(($size-1)) --iterations 100
done
```

### Example 4: Batch Route Optimization
```bash
# Create batch queries
cat > delivery_routes.csv << EOF
start,goal
0,25
0,50
0,75
25,50
25,75
50,75
EOF

# Process all routes
mission9 batch --graph city.json --queries delivery_routes.csv \
  --output delivery_times.csv --algorithm astar

# Analyze results
mission9 --output json batch --graph city.json --queries delivery_routes.csv \
  --algorithm astar > routes.json
```

## Performance Tips

### 1. Algorithm Selection
- **Dijkstra**: Guaranteed optimal, no heuristic needed, slower on large graphs
- **A***: Much faster with good heuristics, requires coordinate data
- **Bidirectional**: Best for grid-based problems, 33x+ speedup on large grids

### 2. File Format Choice
- **JSON**: Best for graphs with coordinates (enables A* heuristics)
- **CSV**: Simpler format, good for sparse graphs without coordinates

### 3. Batch Processing
- Process multiple queries together to amortize graph loading costs
- Use CSV output for easy analysis with spreadsheet tools

### 4. Grid Size Guidelines
- Grids ≤50x50: ASCII visualization available
- Grids 50-500: Use bidirectional algorithms
- Grids >500: Consider preprocessing or hierarchical methods

## Troubleshooting

### No Path Found
```bash
# Check if graph is connected
mission9 info --graph graph.json

# Verify start/goal nodes exist
mission9 --verbose find-path --graph graph.json --start 0 --goal 999
```

### Performance Issues
```bash
# Use verbose mode to see progress
mission9 --verbose find-path --graph large.json --start 0 --goal 10000

# Benchmark to identify bottlenecks
mission9 benchmark --graph large.json --start 0 --goal 10000 --iterations 10
```

### File Format Errors
```bash
# Validate JSON format
cat graph.json | jq .

# Check CSV structure
head queries.csv
```

## Integration Examples

### Python Integration
```python
import subprocess
import json

# Generate graph
subprocess.run([
    "mission9", "generate",
    "--graph-type", "grid",
    "--nodes", "100",
    "--file", "graph.json"
])

# Find path with JSON output
result = subprocess.run([
    "mission9", "--output", "json", "find-path",
    "--graph", "graph.json",
    "--start", "0",
    "--goal", "99",
    "--algorithm", "astar"
], capture_output=True, text=True)

path_data = json.loads(result.stdout)
print(f"Path cost: {path_data['cost']}")
print(f"Path length: {len(path_data['path'])}")
```

### Bash Automation
```bash
#!/bin/bash
# Automated pathfinding pipeline

GRAPH="network.json"
QUERIES="queries.csv"
RESULTS="results.csv"

# Generate graph if needed
if [ ! -f "$GRAPH" ]; then
    mission9 generate --graph-type grid --nodes 400 --file "$GRAPH"
fi

# Run batch processing
mission9 batch --graph "$GRAPH" --queries "$QUERIES" --output "$RESULTS" --algorithm astar

# Extract statistics
echo "Average cost: $(awk -F, 'NR>1 {sum+=$3; count++} END {print sum/count}' $RESULTS)"
echo "Average nodes explored: $(awk -F, 'NR>1 {sum+=$5; count++} END {print sum/count}' $RESULTS)"
```

## Advanced Usage

### Custom Heuristics (via library)
For custom heuristics beyond the CLI, use the library directly:

```rust
use mission9::*;

// Create custom heuristic
struct CustomHeuristic;
impl Heuristic for CustomHeuristic {
    fn estimate(&self, from: u32, to: u32, context: &HeuristicContext) -> f64 {
        // Your custom logic here
        0.0
    }
}

// Use in code
let pathfinder = AstarPathfinder::new(CustomHeuristic);
```

### Multi-threaded Batch Processing
```bash
# Split queries into chunks
split -l 100 queries.csv query_chunk_

# Process in parallel (Unix)
for chunk in query_chunk_*; do
    mission9 batch --graph graph.json --queries "$chunk" --output "result_${chunk}.csv" &
done
wait

# Combine results
cat result_*.csv > all_results.csv
```

## See Also

- [API Documentation](API_DOCUMENTATION.md) - Complete library reference
- [Performance Tuning Guide](PERFORMANCE_TUNING.md) - Optimization strategies
- [Integration Guide](INTEGRATION_GUIDE.md) - Using Mission9 in your projects
