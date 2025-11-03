# Mission 9 - Pathfinding Algorithms Runner Scripts

This directory contains convenient scripts to run the various Mission 9 pathfinding programs with different options.

## Available Scripts

### 1. Batch File (Windows): `run_mission9.bat`
Interactive menu-driven batch file for Windows users.

**Usage:**
```cmd
run_mission9.bat
```

**Features:**
- Interactive menu system
- All available options clearly displayed
- Quick examples for common use cases
- Step-by-step guided examples
- Help system integration

### 2. PowerShell Script: `run_mission9.ps1`
More advanced PowerShell script with parameter support.

**Usage:**
```powershell
# Interactive menu
.\run_mission9.ps1

# Direct commands
.\run_mission9.ps1 graph find-path graph.json 0 5 --algorithm astar
.\run_mission9.ps1 grid solve --width 20 --height 15 --start 0,0 --goal 19,14
.\run_mission9.ps1 demo
.\run_mission9.ps1 bench
```

**Modes:**
- `graph` - Graph-based CLI (mission9)
- `grid` - Grid-based CLI (mission9-grid)
- `demo` - Simple demonstration
- `perf` - Performance analysis demo
- `bench` - Run benchmarks
- `examples` - Show example commands
- `help` - Show detailed help

## Quick Start Examples

### 1. Generate Graph and Find Path
```cmd
# Using batch file - select option 8, then 1
run_mission9.bat

# Using PowerShell
.\run_mission9.ps1 graph generate grid 100 --file test.json
.\run_mission9.ps1 graph find-path test.json 0 99 --algorithm astar --verbose
```

### 2. Grid Pathfinding
```cmd
# Using batch file - select option 2
run_mission9.bat

# Using PowerShell
.\run_mission9.ps1 grid solve --width 15 --height 15 --start 1,1 --goal 13,13 --algorithm astar
```

### 3. Performance Analysis
```cmd
# Using batch file - select option 4
run_mission9.bat

# Using PowerShell
.\run_mission9.ps1 perf
```

### 4. Run Benchmarks
```cmd
# Using batch file - select option 5
run_mission9.bat

# Using PowerShell
.\run_mission9.ps1 bench
```

## Available Programs

### Graph CLI (`mission9`)
- **find-path**: Find shortest path between nodes
- **batch**: Process multiple queries from CSV
- **info**: Display graph information
- **benchmark**: Performance testing
- **generate**: Create test graphs

### Grid CLI (`mission9-grid`)
- **solve**: Pathfinding on 2D grids
- **generate**: Create maze scenarios
- **visualize**: ASCII art output

### Examples
- **simple_demo**: Basic pathfinding demonstration
- **step4_performance_demo**: Advanced performance analysis

## Command Line Arguments

### Graph CLI Arguments
```
mission9 find-path <graph> <start> <goal> [OPTIONS]
  --algorithm <ALGORITHM>    Algorithm to use [astar, dijkstra]
  --heuristic <HEURISTIC>    Heuristic function [manhattan, euclidean]
  --visualize               Enable visualization
  --verbose                 Verbose output

mission9 generate <TYPE> <NODES> [OPTIONS]
  --file <FILE>             Output file
  --format <FORMAT>         Output format [json, csv]
```

### Grid CLI Arguments
```
mission9-grid solve [OPTIONS]
  --width <WIDTH>           Grid width
  --height <HEIGHT>         Grid height
  --start <X,Y>            Start position
  --goal <X,Y>             Goal position
  --algorithm <ALGORITHM>   Algorithm to use
  --obstacles <TYPE>        Obstacle pattern [none, maze, random]
  --visualize              Enable ASCII visualization
```

## File Formats

### Graph Files (JSON)
```json
{
  "nodes": [0, 1, 2, 3],
  "edges": [
    {"from": 0, "to": 1, "weight": 1.0},
    {"from": 1, "to": 2, "weight": 1.5}
  ]
}
```

### Query Files (CSV)
```csv
start,goal
0,5
2,8
10,15
```

## Tips

1. **Start with the batch file** if you're new - it provides guided examples
2. **Use PowerShell script** for automation or when you know the exact commands
3. **Check help** with `--help` flag for detailed options
4. **Generate test data** first before running pathfinding algorithms
5. **Use verbose mode** to see detailed algorithm execution

## Performance Notes

- Grid CLI is optimized for 2D spatial problems
- Graph CLI handles general graph structures
- Benchmarks provide detailed performance metrics
- Use A* for goal-oriented searches
- Use Dijkstra when exploring all paths or when heuristics aren't available

## Troubleshooting

1. **Cargo not found**: Make sure Rust is installed and in PATH
2. **Permission errors**: Run PowerShell as administrator if needed
3. **File not found**: Check that you're in the Mission9 directory
4. **Build errors**: Run `cargo build` first to compile

---
## Related Resources
- [[missions/Mission9/README|Mission 9 Overview]] - Full pathfinding implementation guide
- [[missions/Mission9/LAUNCHER_TEST_SUMMARY|Launcher Test Summary]] - CLI interface and batch scripts
- [[missions/Mission9/day7_completion_summary|Day 7 Summary]] - Final completion and integration
- [[zettelkasten/Missions Overview|Missions Overview]] - Full mission series navigation
- [[zettelkasten/mission-9|Mission 9 Zettel]] - Pathfinding algorithms deep dive

*Tags: #mission9 #benchmarking #performance #cli-tools #testing #runner-scripts*