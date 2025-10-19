//! Mission 8 Tutorial - Step 4: Performance Benchmarking
//! 
//! This tutorial teaches you how to benchmark Rust code using Criterion,
//! measure algorithm performance, and interpret benchmark results.

use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Mission 8 Tutorial - Step 4: Performance Benchmarking ===\n");
    
    section1_why_benchmark();
    section2_basic_timing();
    section3_criterion_setup();
    section4_graph_creation();
    section5_benchmarking_bfs_dfs();
    section6_interpreting_results();
    section7_optimization_strategies();
    
    println!("🎉 Step 4 Complete! You now understand:");
    println!("  ✅ Why benchmarking is important");
    println!("  ✅ How to measure execution time");
    println!("  ✅ Setting up Criterion benchmarks");
    println!("  ✅ Creating graphs for performance testing");
    println!("  ✅ Comparing algorithm performance");
    println!("  ✅ Interpreting benchmark results");
    println!("  ✅ Basic optimization strategies");
    println!();
    println!("Next: Step 5 - Real-World Applications (Maze Solver)");
}

fn section1_why_benchmark() {
    println!("📚 Section 1: Why Benchmark?");
    println!("============================");
    
    println!("🎯 Why Performance Matters:");
    println!("  • Big-O notation tells us scaling behavior, not actual performance");
    println!("  • Real-world performance depends on constants, cache effects, etc.");
    println!("  • \"Premature optimization is the root of all evil\" - but measurement isn't!");
    println!("  • Benchmarks help validate theoretical complexity claims");
    println!();
    
    println!("🎯 When to Benchmark:");
    println!("  ✅ After implementing core algorithms (like we're doing now)");
    println!("  ✅ When performance is critical for your application"); 
    println!("  ✅ Before and after optimizations (to verify improvements)");
    println!("  ✅ To compare different algorithmic approaches");
    println!();
    
    println!("🎯 What We'll Measure:");
    println!("  • BFS vs DFS execution time");
    println!("  • How algorithms scale with graph size");
    println!("  • Memory usage patterns");
    println!("  • Impact of graph structure (sparse vs dense)");
    println!();
}

fn section2_basic_timing() {
    println!("⏱️  Section 2: Basic Timing Measurements");
    println!("========================================");
    
    println!("🎯 Simple Timing with std::time::Instant:");
    
    // Create a simple graph for demonstration
    let mut graph = HashMap::new();
    for i in 0..1000 {
        graph.insert(i, vec![(i + 1) % 1000, (i + 500) % 1000]);
    }
    
    // Measure BFS execution time
    let start = Instant::now();
    let _bfs_result = tutorial_bfs(&graph, 0);
    let bfs_duration = start.elapsed();
    
    println!("  BFS on 1000-node graph: {:?}", bfs_duration);
    
    // Measure DFS execution time  
    let start = Instant::now();
    let _dfs_result = tutorial_dfs(&graph, 0);
    let dfs_duration = start.elapsed();
    
    println!("  DFS on 1000-node graph: {:?}", dfs_duration);
    
    println!();
    println!("🎯 Timing Best Practices:");
    println!("  • Run multiple iterations to reduce noise");
    println!("  • Use release mode for accurate performance");
    println!("  • Warm up the code before measuring");
    println!("  • Account for system variance");
    println!();
    
    // Demonstrate multiple runs
    let mut times = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let _result = tutorial_bfs(&graph, 0);
        times.push(start.elapsed());
    }
    
    let avg_time: Duration = times.iter().sum::<Duration>() / times.len() as u32;
    let min_time = times.iter().min().unwrap();
    let max_time = times.iter().max().unwrap();
    
    println!("🧪 Multiple BFS runs on 1000 nodes:");
    println!("  Average: {:?}", avg_time);
    println!("  Min: {:?}", min_time);
    println!("  Max: {:?}", max_time);
    println!("  Variance shows why we need statistical measurement!");
    println!();
}

fn section3_criterion_setup() {
    println!("📊 Section 3: Setting Up Criterion");
    println!("==================================");
    
    println!("🎯 Why Criterion?");
    println!("  • Statistical rigor - measures confidence intervals");
    println!("  • Automatic warmup and multiple iterations");
    println!("  • Regression detection between runs");
    println!("  • Beautiful HTML reports");
    println!("  • Prevents compiler optimizations with `black_box()`");
    println!();
    
    println!("🎯 Cargo.toml Configuration:");
    println!("```toml");
    println!("[dev-dependencies]");
    println!("criterion = \"0.5\"");
    println!();
    println!("[[bench]]");
    println!("name = \"algorithm_benchmarks\"");
    println!("harness = false");
    println!("```");
    println!();
    
    println!("🎯 Basic Criterion Benchmark Structure:");
    println!("```rust");
    println!("use criterion::{{black_box, criterion_group, criterion_main, Criterion}};");
    println!();
    println!("fn bench_bfs(c: &mut Criterion) {{");
    println!("    let graph = create_test_graph(1000);");
    println!("    c.bench_function(\"bfs_1000_nodes\", |b| {{");
    println!("        b.iter(|| bfs(black_box(&graph), black_box(0)))");
    println!("    }});");
    println!("}}");
    println!();
    println!("criterion_group!(benches, bench_bfs);");
    println!("criterion_main!(benches);");
    println!("```");
    println!();
    
    println!("🎯 Running Benchmarks:");
    println!("  cargo bench -p mission8");
    println!("  Open target/criterion/index.html for detailed results");
    println!();
}

fn section4_graph_creation() {
    println!("🏗️  Section 4: Creating Test Graphs");
    println!("====================================");
    
    println!("🎯 Different Graph Types for Testing:");
    
    // Linear chain - best case for some algorithms
    println!("  📏 Linear Chain (10 nodes): 0 -> 1 -> 2 -> ... -> 9");
    let linear = create_linear_graph(10);
    println!("     Nodes: {}, Edges: {}", count_nodes(&linear), count_edges(&linear));
    
    // Complete graph - worst case 
    println!("  🌐 Complete Graph (5 nodes): every node connected to every other");
    let complete = create_complete_graph(5);
    println!("     Nodes: {}, Edges: {}", count_nodes(&complete), count_edges(&complete));
    
    // Sparse graph - realistic case
    println!("  🕸️  Sparse Graph (100 nodes): each node has ~3 connections");
    let sparse = create_sparse_graph(100);
    println!("     Nodes: {}, Edges: {}", count_nodes(&sparse), count_edges(&sparse));
    
    // Dense graph - stress test
    println!("  🧱 Dense Graph (50 nodes): each node has ~20 connections");
    let dense = create_dense_graph(50);
    println!("     Nodes: {}, Edges: {}", count_nodes(&dense), count_edges(&dense));
    
    println!();
    println!("🎯 Why Different Graph Types Matter:");
    println!("  • Linear: Tests best-case performance");
    println!("  • Complete: Tests worst-case performance (lots of neighbors)");
    println!("  • Sparse: Tests realistic web/social network scenarios");
    println!("  • Dense: Tests performance under high connectivity");
    println!();
}

fn section5_benchmarking_bfs_dfs() {
    println!("🏃 Section 5: Benchmarking BFS vs DFS");
    println!("=====================================");
    
    println!("🎯 Performance Comparison on Different Graph Sizes:");
    
    let sizes = vec![100, 500, 1000, 2000];
    
    println!("┌─────────┬─────────────┬─────────────┬──────────────┐");
    println!("│  Size   │  BFS Time   │  DFS Time   │   Ratio      │");
    println!("├─────────┼─────────────┼─────────────┼──────────────┤");
    
    for &size in &sizes {
        let graph = create_sparse_graph(size);
        
        // Benchmark BFS
        let bfs_times: Vec<Duration> = (0..5)
            .map(|_| {
                let start = Instant::now();
                let _result = tutorial_bfs(&graph, 0);
                start.elapsed()
            })
            .collect();
        let avg_bfs = bfs_times.iter().sum::<Duration>() / bfs_times.len() as u32;
        
        // Benchmark DFS
        let dfs_times: Vec<Duration> = (0..5)
            .map(|_| {
                let start = Instant::now();
                let _result = tutorial_dfs(&graph, 0);
                start.elapsed()
            })
            .collect();
        let avg_dfs = dfs_times.iter().sum::<Duration>() / dfs_times.len() as u32;
        
        let ratio = avg_bfs.as_nanos() as f64 / avg_dfs.as_nanos() as f64;
        
        println!("│ {:>7} │ {:>9.2?} │ {:>9.2?} │ {:>9.2}x │", 
                size, avg_bfs, avg_dfs, ratio);
    }
    
    println!("└─────────┴─────────────┴─────────────┴──────────────┘");
    println!();
    
    println!("🎯 Performance Analysis:");
    println!("  • Both algorithms are O(V + E) - should scale linearly");
    println!("  • BFS uses queue (VecDeque) - good cache locality");
    println!("  • DFS uses stack (Vec) - potentially better memory usage");
    println!("  • Performance differences often come from constant factors");
    println!("  • Graph structure matters more than algorithm choice!");
    println!();
    
    println!("🎯 Scaling Verification:");
    println!("  Graph size doubled from 1000 to 2000 nodes");
    println!("  If algorithms are truly O(V + E), time should roughly double");
    println!("  Large deviations suggest non-linear behavior or system interference");
    println!();
}

fn section6_interpreting_results() {
    println!("📈 Section 6: Interpreting Benchmark Results");
    println!("============================================");
    
    println!("🎯 What Criterion Measures:");
    println!("  • **Mean**: Average execution time");
    println!("  • **Standard Deviation**: Variability in measurements");
    println!("  • **Confidence Intervals**: Range where true value likely lies");
    println!("  • **Outliers**: Measurements excluded from statistics");
    println!("  • **R²**: How well a linear model fits the data");
    println!();
    
    println!("🎯 Reading Criterion Output:");
    println!("```");
    println!("bfs_1000_nodes          time:   [125.67 us 126.45 us 127.89 us]");
    println!("                        change: [-2.1% -0.8% +1.5%] (p = 0.45 > 0.05)");
    println!("                        No change in performance detected.");
    println!("```");
    println!();
    println!("  • Time range: [lower bound, estimate, upper bound]");
    println!("  • Change: Comparison to previous run (if available)");
    println!("  • p-value: Statistical significance of change");
    println!();
    
    println!("🎯 Red Flags to Watch For:");
    println!("  ❌ **High Standard Deviation**: System under load, inconsistent performance");
    println!("  ❌ **Many Outliers**: Background processes interfering");
    println!("  ❌ **Poor R² (<0.9)**: Algorithm might not be scaling linearly");
    println!("  ❌ **Unexpected Scaling**: O(n) algorithm taking O(n²) time");
    println!();
    
    println!("🎯 Good Benchmark Practices:");
    println!("  ✅ Run on dedicated machine when possible");
    println!("  ✅ Use release mode: `cargo criterion --profile release`");
    println!("  ✅ Warm up caches with multiple iterations");
    println!("  ✅ Compare relative performance, not absolute times");
    println!("  ✅ Test multiple graph sizes to verify scaling");
}

fn section7_optimization_strategies() {
    println!("⚡ Section 7: Optimization Strategies");
    println!("====================================");
    
    println!("🎯 Data Structure Choices:");
    println!("  • HashSet vs Vec for visited tracking");
    println!("  • VecDeque vs Vec for queues");
    println!("  • HashMap vs Vec for adjacency representation");
    println!();
    
    // Demonstrate visited tracking comparison
    let graph = create_sparse_graph(1000);
    
    println!("🧪 Visited Tracking Comparison:");
    
    // BFS with HashSet (current implementation)
    let start = Instant::now();
    let _result1 = tutorial_bfs(&graph, 0);
    let hashset_time = start.elapsed();
    
    // BFS with Vec (alternative - need to implement)
    let start = Instant::now();
    let _result2 = tutorial_bfs_with_vec(&graph, 0, 1000);
    let vec_time = start.elapsed();
    
    println!("  HashSet visited: {:?}", hashset_time);
    println!("  Vec visited:     {:?}", vec_time);
    println!("  Ratio: {:.2}x", hashset_time.as_nanos() as f64 / vec_time.as_nanos() as f64);
    println!();
    
    println!("🎯 Memory Pre-allocation:");
    println!("  • Use `Vec::with_capacity()` when size is known");
    println!("  • Use `HashMap::with_capacity()` for better performance");
    println!("  • Pre-allocate reduces memory allocations during algorithm");
    println!();
    
    println!("🎯 Algorithm-Specific Optimizations:");
    println!("  **BFS Optimizations:**");
    println!("  • Use `VecDeque::with_capacity()` for queue");
    println!("  • Consider bidirectional BFS for shortest path");
    println!("  • Early termination when target found");
    println!();
    println!("  **DFS Optimizations:**");
    println!("  • Iterative vs recursive (avoid stack overflow)");
    println!("  • Tail recursion optimization");
    println!("  • Path compression in union-find applications");
    println!();
    
    println!("🎯 Profile-Guided Optimization:");
    println!("  1. **Measure first** - identify actual bottlenecks");
    println!("  2. **Optimize hot paths** - focus on most-called code");
    println!("  3. **Measure again** - verify improvements");
    println!("  4. **Consider cache effects** - data locality matters");
    println!();
    
    println!("⚠️  Optimization Warnings:");
    println!("  • \"Premature optimization is the root of all evil\"");
    println!("  • Readability vs performance trade-offs");
    println!("  • Micro-optimizations vs algorithmic improvements");
    println!("  • Always benchmark before and after changes");
    println!();
}

// Helper functions for the tutorial

fn tutorial_bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) -> Vec<u32> {
    use std::collections::{HashSet, VecDeque};
    
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        result.push(current);
        
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }
    
    result
}

fn tutorial_dfs(graph: &HashMap<u32, Vec<u32>>, start: u32) -> Vec<u32> {
    use std::collections::HashSet;
    
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut result = Vec::new();
    
    stack.push(start);
    
    while let Some(current) = stack.pop() {
        if !visited.contains(&current) {
            visited.insert(current);
            result.push(current);
            
            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors.iter().rev() { // Reverse for consistent ordering
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    
    result
}

fn tutorial_bfs_with_vec(graph: &HashMap<u32, Vec<u32>>, start: u32, max_node: u32) -> Vec<u32> {
    use std::collections::VecDeque;
    
    let mut visited = vec![false; (max_node + 1) as usize];
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    queue.push_back(start);
    visited[start as usize] = true;
    
    while let Some(current) = queue.pop_front() {
        result.push(current);
        
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !visited[neighbor as usize] {
                    visited[neighbor as usize] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }
    
    result
}

fn create_linear_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        if i < size - 1 {
            graph.insert(i, vec![i + 1]);
        } else {
            graph.insert(i, vec![]);
        }
    }
    graph
}

fn create_complete_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        let neighbors: Vec<u32> = (0..size).filter(|&x| x != i).collect();
        graph.insert(i, neighbors);
    }
    graph
}

fn create_sparse_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        let neighbors = vec![
            (i + 1) % size,
            (i + size / 2) % size,
            (i + size / 4) % size,
        ];
        graph.insert(i, neighbors);
    }
    graph
}

fn create_dense_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        let mut neighbors = Vec::new();
        for j in 1..=(size / 3) {  // Connect to about 1/3 of all other nodes
            neighbors.push((i + j) % size);
            neighbors.push((i + size - j) % size);
        }
        neighbors.sort();
        neighbors.dedup();
        graph.insert(i, neighbors);
    }
    graph
}

fn count_nodes(graph: &HashMap<u32, Vec<u32>>) -> usize {
    graph.len()
}

fn count_edges(graph: &HashMap<u32, Vec<u32>>) -> usize {
    graph.values().map(|neighbors| neighbors.len()).sum()
}