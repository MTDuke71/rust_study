//! # Step 4: Performance Optimization Demo
//!
//! This example demonstrates the performance improvements achieved through:
//! - REQ-3: Bidirectional search (A* and Dijkstra)
//! - REQ-4: Early termination strategies
//! - Memory optimization with node pools
//! - Performance measurement and comparison

use mission9::performance_optimization::{BidirectionalAstar, BidirectionalDijkstra, MemoryOptimizedAstar, PerformanceMetrics};
use std::collections::HashSet;

/// Simple grid representation for pathfinding
#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: HashSet<(usize, usize)>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }

    fn add_obstacle(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.obstacles.insert((x, y));
        }
    }

    fn add_wall(&mut self, start: (usize, usize), end: (usize, usize)) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        
        if start_x == end_x {
            // Vertical wall
            let min_y = start_y.min(end_y);
            let max_y = start_y.max(end_y);
            for y in min_y..=max_y {
                self.add_obstacle(start_x, y);
            }
        } else if start_y == end_y {
            // Horizontal wall
            let min_x = start_x.min(end_x);
            let max_x = start_x.max(end_x);
            for x in min_x..=max_x {
                self.add_obstacle(x, start_y);
            }
        }
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<((usize, usize), u32)> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;

        // 4-way movement with unit cost
        if x > 0 && !self.obstacles.contains(&(x - 1, y)) {
            neighbors.push(((x - 1, y), 1));
        }
        if x + 1 < self.width && !self.obstacles.contains(&(x + 1, y)) {
            neighbors.push(((x + 1, y), 1));
        }
        if y > 0 && !self.obstacles.contains(&(x, y - 1)) {
            neighbors.push(((x, y - 1), 1));
        }
        if y + 1 < self.height && !self.obstacles.contains(&(x, y + 1)) {
            neighbors.push(((x, y + 1), 1));
        }

        neighbors
    }

    fn display_with_path(&self, path: Option<&Vec<(usize, usize)>>, start: (usize, usize), goal: (usize, usize)) {
        let path_set: HashSet<(usize, usize)> = path
            .map(|p| p.iter().cloned().collect())
            .unwrap_or_default();

        println!("\nGrid Visualization:");
        println!("S = Start, G = Goal, # = Obstacle, * = Path, . = Empty");
        
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                let symbol = if pos == start {
                    'S'
                } else if pos == goal {
                    'G'
                } else if self.obstacles.contains(&pos) {
                    '#'
                } else if path_set.contains(&pos) {
                    '*'
                } else {
                    '.'
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}

/// Manhattan distance heuristic
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u32 {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    (dx + dy) as u32
}

/// Euclidean distance heuristic (scaled for integer math)
fn euclidean_distance(a: (usize, usize), b: (usize, usize)) -> u32 {
    let dx = (a.0 as i32 - b.0 as i32) as f64;
    let dy = (a.1 as i32 - b.1 as i32) as f64;
    (dx * dx + dy * dy).sqrt() as u32
}

/// Benchmark result structure
#[derive(Debug)]
#[allow(dead_code)]
struct BenchmarkResult {
    algorithm: String,
    path_found: bool,
    path_length: usize,
    path_cost: u32,
    nodes_explored: usize,
    nodes_generated: usize,
    early_terminations: usize,
    search_time_ns: u128,
    memory_efficiency: f64,
}

impl BenchmarkResult {
    fn from_metrics(algorithm: String, metrics: &PerformanceMetrics, path_found: bool) -> Self {
        let memory_efficiency = if metrics.nodes_generated > 0 {
            metrics.nodes_explored as f64 / metrics.nodes_generated as f64
        } else {
            0.0
        };

        Self {
            algorithm,
            path_found,
            path_length: metrics.path_length,
            path_cost: metrics.path_cost,
            nodes_explored: metrics.nodes_explored,
            nodes_generated: metrics.nodes_generated,
            early_terminations: metrics.early_terminations,
            search_time_ns: metrics.search_time_ns,
            memory_efficiency,
        }
    }
}

fn create_complex_maze() -> Grid {
    let mut grid = Grid::new(40, 30);
    
    // Create a maze-like structure with multiple paths
    
    // Outer walls
    for x in 0..grid.width {
        grid.add_obstacle(x, 0);
        grid.add_obstacle(x, grid.height - 1);
    }
    for y in 0..grid.height {
        grid.add_obstacle(0, y);
        grid.add_obstacle(grid.width - 1, y);
    }
    
    // Internal maze structure
    grid.add_wall((5, 5), (35, 5));   // Horizontal barrier
    grid.add_wall((5, 15), (35, 15)); // Another horizontal barrier
    grid.add_wall((5, 25), (35, 25)); // Bottom horizontal barrier
    
    grid.add_wall((10, 1), (10, 10)); // Vertical barriers
    grid.add_wall((20, 5), (20, 20));
    grid.add_wall((30, 10), (30, 28));
    
    // Create openings in walls
    grid.obstacles.remove(&(15, 5));
    grid.obstacles.remove(&(25, 5));
    grid.obstacles.remove(&(12, 15));
    grid.obstacles.remove(&(28, 15));
    grid.obstacles.remove(&(18, 25));
    
    // Add some scattered obstacles
    for &(x, y) in &[(7, 8), (23, 12), (17, 18), (32, 22), (8, 20)] {
        grid.add_obstacle(x, y);
    }
    
    grid
}

fn create_simple_grid() -> Grid {
    let mut grid = Grid::new(15, 15);
    
    // Simple obstacle pattern
    for i in 5..10 {
        grid.add_obstacle(i, 7);
        grid.add_obstacle(7, i);
    }
    
    grid
}

fn run_bidirectional_astar_manhattan(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
) -> BenchmarkResult {
    let mut astar = BidirectionalAstar::new(10000);
    let path = astar.find_path(start, goal, manhattan_distance, |pos| grid.neighbors(pos));
    BenchmarkResult::from_metrics("Bidirectional A* (Manhattan)".to_string(), astar.get_metrics(), path.is_some())
}

fn run_bidirectional_astar_euclidean(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
) -> BenchmarkResult {
    let mut astar = BidirectionalAstar::new(10000);
    let path = astar.find_path(start, goal, euclidean_distance, |pos| grid.neighbors(pos));
    BenchmarkResult::from_metrics("Bidirectional A* (Euclidean)".to_string(), astar.get_metrics(), path.is_some())
}

fn run_bidirectional_dijkstra(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
) -> BenchmarkResult {
    let mut dijkstra = BidirectionalDijkstra::new(10000);
    let path = dijkstra.find_path(start, goal, |pos| grid.neighbors(pos));
    BenchmarkResult::from_metrics("Bidirectional Dijkstra".to_string(), dijkstra.get_metrics(), path.is_some())
}

fn run_memory_optimized_astar(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
) -> BenchmarkResult {
    let mut mem_astar = MemoryOptimizedAstar::new(10000);
    let path = mem_astar.find_path(start, goal, manhattan_distance, |pos| grid.neighbors(pos));
    BenchmarkResult::from_metrics("Memory-Optimized A*".to_string(), mem_astar.get_metrics(), path.is_some())
}

fn main() {
    println!("🚀 Mission 9 - Step 4: Performance Optimization Demo");
    println!("====================================================\n");

    // Test scenarios
    let scenarios = vec![
        ("Simple Grid (15x15)", create_simple_grid(), (2, 2), (12, 12)),
        ("Complex Maze (40x30)", create_complex_maze(), (2, 2), (37, 27)),
    ];

    for (scenario_name, grid, start, goal) in scenarios {
        println!("📋 Scenario: {}", scenario_name);
        println!("   Grid size: {}x{}", grid.width, grid.height);
        println!("   Start: {:?}, Goal: {:?}", start, goal);
        println!("   Obstacles: {}", grid.obstacles.len());

        // Display the grid for smaller scenarios
        if grid.width <= 20 {
            grid.display_with_path(None, start, goal);
        }

        // Benchmark all algorithms
        let results = vec![
            run_bidirectional_astar_manhattan(&grid, start, goal),
            run_bidirectional_astar_euclidean(&grid, start, goal),
            run_bidirectional_dijkstra(&grid, start, goal),
            run_memory_optimized_astar(&grid, start, goal),
        ];

        // Display results table
        println!("\n📊 Performance Comparison:");
        println!("{:<25} | {:>8} | {:>10} | {:>12} | {:>12} | {:>8} | {:>12}", 
                 "Algorithm", "Found", "Path Len", "Nodes Exp", "Nodes Gen", "Early T", "Time (μs)");
        println!("{:-<25}-|-{:->8}-|-{:->10}-|-{:->12}-|-{:->12}-|-{:->8}-|-{:->12}",
                 "", "", "", "", "", "", "");

        for result in &results {
            println!("{:<25} | {:>8} | {:>10} | {:>12} | {:>12} | {:>8} | {:>12}",
                     result.algorithm,
                     if result.path_found { "✅" } else { "❌" },
                     result.path_length,
                     result.nodes_explored,
                     result.nodes_generated,
                     result.early_terminations,
                     result.search_time_ns / 1000);
        }

        // Performance analysis
        if results.len() >= 2 {
            let bidirectional = &results[0];
            let dijkstra = results.iter().find(|r| r.algorithm.contains("Dijkstra")).unwrap();
            
            if bidirectional.nodes_explored > 0 && dijkstra.nodes_explored > 0 {
                let exploration_improvement = 100.0 * (1.0 - bidirectional.nodes_explored as f64 / dijkstra.nodes_explored as f64);
                let time_improvement = 100.0 * (1.0 - bidirectional.search_time_ns as f64 / dijkstra.search_time_ns as f64);
                
                println!("\n🎯 Bidirectional A* vs Dijkstra Improvements:");
                println!("   Exploration reduction: {:.1}%", exploration_improvement);
                println!("   Time reduction: {:.1}%", time_improvement);
                println!("   Early terminations: {}", bidirectional.early_terminations);
            }
        }

        // Show path for smaller grids
        if grid.width <= 20 && !results.is_empty() && results[0].path_found {
            let mut astar = BidirectionalAstar::new(1000);
            if let Some(path) = astar.find_path(start, goal, manhattan_distance, |pos| grid.neighbors(pos)) {
                grid.display_with_path(Some(&path), start, goal);
            }
        }

        println!("\n{}\n", "=".repeat(80));
    }

    // Memory optimization demonstration
    println!("💾 Memory Optimization Analysis:");
    println!("===============================");

    println!("📏 Node Structure Sizes:");
    println!("   Current OptimizedNode: {} bytes", std::mem::size_of::<mission9::performance_optimization::OptimizedNode>());
    println!("   TrulyOptimizedNode (u32): {} bytes", std::mem::size_of::<mission9::performance_optimization::TrulyOptimizedNode>());
    println!("   Standard (usize-based): {} bytes", std::mem::size_of::<(usize, usize, u32, u32, Option<(usize, usize)>)>());
    
    let size_reduction_truly = 100.0 * (1.0 - std::mem::size_of::<mission9::performance_optimization::TrulyOptimizedNode>() as f64 / 
                                 std::mem::size_of::<(usize, usize, u32, u32, Option<(usize, usize)>)>() as f64);
    println!("   💡 Memory reduction with u32 optimization: {:.1}%", size_reduction_truly);
    
    let memory_per_1000_nodes_truly_optimized = std::mem::size_of::<mission9::performance_optimization::TrulyOptimizedNode>() * 1000;
    let memory_per_1000_nodes_current = std::mem::size_of::<mission9::performance_optimization::OptimizedNode>() * 1000;
    let memory_per_1000_nodes_standard = std::mem::size_of::<(usize, usize, u32, u32, Option<(usize, usize)>)>() * 1000;
    
    println!("   📊 Memory usage for 1000 nodes:");
    println!("      Current Optimized: {} KB", memory_per_1000_nodes_current / 1024);
    println!("      Truly Optimized:   {} KB", memory_per_1000_nodes_truly_optimized / 1024);
    println!("      Standard:          {} KB", memory_per_1000_nodes_standard / 1024);
    println!("      Potential Savings: {} KB", (memory_per_1000_nodes_standard - memory_per_1000_nodes_truly_optimized) / 1024);
    
    println!("\n🔬 Memory Layout Analysis:");
    println!("   Current OptimizedNode breakdown:");
    println!("   • position: (usize, usize) = {} bytes", std::mem::size_of::<(usize, usize)>());
    println!("   • g_cost: u32 = {} bytes", std::mem::size_of::<u32>());
    println!("   • h_cost: u32 = {} bytes", std::mem::size_of::<u32>());
    println!("   • parent: Option<(usize, usize)> = {} bytes", std::mem::size_of::<Option<(usize, usize)>>());
    
    println!("\n   TrulyOptimizedNode breakdown:");
    println!("   • position: (u32, u32) = {} bytes", std::mem::size_of::<(u32, u32)>());
    println!("   • g_cost: u32 = {} bytes", std::mem::size_of::<u32>());
    println!("   • h_cost: u32 = {} bytes", std::mem::size_of::<u32>());
    println!("   • parent: Option<(u32, u32)> = {} bytes", std::mem::size_of::<Option<(u32, u32)>>());

    println!("\n🔄 Node Pool Benefits:");
    println!("   • Reduced allocation overhead");
    println!("   • Better cache locality");
    println!("   • Memory reuse and recycling");  
    println!("   • Predictable memory usage patterns");
    println!("   • Lower garbage collection pressure");

    // REQ-3 and REQ-4 summary
    println!("\n✅ Requirements Implementation Summary:");
    println!("=====================================");
    println!("🎯 REQ-3: Bidirectional Search Optimization");
    println!("   ✅ Bidirectional A* algorithm implemented");
    println!("   ✅ Bidirectional Dijkstra algorithm implemented");
    println!("   ✅ Meeting point detection with optimal path reconstruction");
    println!("   ✅ Simultaneous forward and backward search expansion");

    println!("\n🎯 REQ-4: Early Termination Strategies");
    println!("   ✅ F-score bound checking for early termination");
    println!("   ✅ Meeting point cost comparison");
    println!("   ✅ Combined search bound optimization");
    println!("   ✅ Performance metrics tracking for termination analysis");

    println!("\n🏆 Performance Optimization Features:");
    println!("   • Memory-optimized node structures (u32 vs usize)");
    println!("   • Node pool allocation for memory reuse");
    println!("   • Cache-friendly data layout");
    println!("   • Comprehensive performance metrics");
    println!("   • Configurable heuristic functions");

    println!("\n📈 Expected Performance Gains:");
    println!("   • 50-80% reduction in nodes explored (bidirectional)");
    println!("   • 30-60% reduction in search time");
    println!("   • Reduced memory fragmentation");
    println!("   • Better scalability for large graphs");

    println!("\n🔗 Next Steps:");
    println!("   • Step 5: Advanced heuristics and multi-objective optimization");
    println!("   • Hierarchical pathfinding for massive graphs");
    println!("   • Parallel bidirectional search implementation");
    println!("   • Jump Point Search (JPS) for grid optimization");
}