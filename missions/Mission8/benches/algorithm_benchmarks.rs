use criterion::{criterion_group, criterion_main, Criterion};
use mission8::{bfs, connected_components, dfs, find_cycle, has_cycle, shortest_path};
use std::collections::{HashMap, HashSet};
use std::hint::black_box;

/// Create a small graph (10 nodes) for quick benchmarks
fn create_small_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    // Linear chain: 0 -> 1 -> 2 -> ... -> 9
    for i in 0..9 {
        graph.insert(i, vec![i + 1]);
    }
    graph.insert(9, vec![]); // End node
    graph
}

/// Create a medium graph (1000 nodes) for scalability testing
fn create_medium_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..1000 {
        let neighbors = if i < 999 {
            vec![i + 1, (i + 2) % 1000] // Each node connects to next and one other
        } else {
            vec![0, 500] // Last node connects back
        };
        graph.insert(i, neighbors);
    }
    graph
}

/// Create a large graph (10,000 nodes) for performance testing
fn create_large_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..10000 {
        let neighbors = if i < 9999 {
            vec![i + 1, (i + 500) % 10000, (i + 1000) % 10000]
        } else {
            vec![0, 5000, 2500]
        };
        graph.insert(i, neighbors);
    }
    graph
}

/// Create a complete graph (all nodes connected to all others) for worst-case
fn create_complete_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        let neighbors: Vec<u32> = (0..size).filter(|&x| x != i).collect();
        graph.insert(i, neighbors);
    }
    graph
}

/// Recursive DFS implementation for comparison with iterative version
fn dfs_recursive_helper<N>(
    graph: &HashMap<N, Vec<N>>,
    node: N,
    visited: &mut HashSet<N>,
    result: &mut Vec<N>,
) where
    N: Copy + Eq + std::hash::Hash,
{
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);
    result.push(node);

    for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
        dfs_recursive_helper(graph, neighbor, visited, result);
    }
}

fn dfs_recursive<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
where
    N: Copy + Eq + std::hash::Hash,
{
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    dfs_recursive_helper(graph, start, &mut visited, &mut result);
    result
}

/// Create a cyclic graph for cycle detection benchmarks
fn create_cyclic_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        graph.insert(i, vec![(i + 1) % size]); // Creates a cycle
    }
    graph
}

/// Create a disconnected graph with multiple components
fn create_disconnected_graph(components: u32, size_per_component: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for comp in 0..components {
        let base = comp * size_per_component;
        for i in 0..size_per_component {
            let node = base + i;
            let next = base + ((i + 1) % size_per_component);
            graph.insert(node, vec![next]);
        }
    }
    graph
}

fn bench_bfs_small_graph(c: &mut Criterion) {
    let graph = create_small_graph();
    c.bench_function("bfs_small_graph", |b| {
        b.iter(|| bfs(black_box(&graph), black_box(0)))
    });
}

fn bench_bfs_medium_graph(c: &mut Criterion) {
    let graph = create_medium_graph();
    c.bench_function("bfs_medium_graph", |b| {
        b.iter(|| bfs(black_box(&graph), black_box(0)))
    });
}

fn bench_bfs_large_graph(c: &mut Criterion) {
    let graph = create_large_graph();
    c.bench_function("bfs_large_graph", |b| {
        b.iter(|| bfs(black_box(&graph), black_box(0)))
    });
}

fn bench_dfs_small_graph(c: &mut Criterion) {
    let graph = create_small_graph();
    c.bench_function("dfs_small_graph", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });
}

fn bench_dfs_medium_graph(c: &mut Criterion) {
    let graph = create_medium_graph();
    c.bench_function("dfs_medium_graph", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });
}

fn bench_dfs_large_graph(c: &mut Criterion) {
    let graph = create_large_graph();
    c.bench_function("dfs_large_graph", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });
}

// ============================================================================
// RECURSIVE vs ITERATIVE DFS COMPARISON BENCHMARKS
// ============================================================================

fn bench_dfs_iterative_vs_recursive_small(c: &mut Criterion) {
    let graph = create_small_graph();
    let mut group = c.benchmark_group("dfs_small_recursive_vs_iterative");

    group.bench_function("iterative_dfs", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });

    group.bench_function("recursive_dfs", |b| {
        b.iter(|| dfs_recursive(black_box(&graph), black_box(0)))
    });

    group.finish();
}

fn bench_dfs_iterative_vs_recursive_medium(c: &mut Criterion) {
    let graph = create_medium_graph();
    let mut group = c.benchmark_group("dfs_medium_recursive_vs_iterative");

    group.bench_function("iterative_dfs", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });

    group.bench_function("recursive_dfs", |b| {
        b.iter(|| dfs_recursive(black_box(&graph), black_box(0)))
    });

    group.finish();
}

fn bench_dfs_iterative_vs_recursive_large(c: &mut Criterion) {
    let graph = create_large_graph();
    let mut group = c.benchmark_group("dfs_large_recursive_vs_iterative");

    group.bench_function("iterative_dfs", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });

    group.bench_function("recursive_dfs", |b| {
        b.iter(|| dfs_recursive(black_box(&graph), black_box(0)))
    });

    group.finish();
}

/// Create a deep linear graph to test stack overflow resistance
fn create_deep_linear_graph(depth: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..depth - 1 {
        graph.insert(i, vec![i + 1]);
    }
    graph.insert(depth - 1, vec![]); // End node
    graph
}

fn bench_dfs_deep_graph_stack_safety(c: &mut Criterion) {
    let shallow_graph = create_deep_linear_graph(100);
    let deep_graph = create_deep_linear_graph(1000);
    let very_deep_graph = create_deep_linear_graph(5000);

    let mut group = c.benchmark_group("dfs_stack_safety");

    // Shallow graph - both should work fine
    group.bench_function("shallow_iterative", |b| {
        b.iter(|| dfs(black_box(&shallow_graph), black_box(0)))
    });

    group.bench_function("shallow_recursive", |b| {
        b.iter(|| dfs_recursive(black_box(&shallow_graph), black_box(0)))
    });

    // Deep graph - recursive might start to show overhead
    group.bench_function("deep_iterative", |b| {
        b.iter(|| dfs(black_box(&deep_graph), black_box(0)))
    });

    group.bench_function("deep_recursive", |b| {
        b.iter(|| dfs_recursive(black_box(&deep_graph), black_box(0)))
    });

    // Very deep graph - only iterative should handle this well
    group.bench_function("very_deep_iterative", |b| {
        b.iter(|| dfs(black_box(&very_deep_graph), black_box(0)))
    });

    // Note: Commenting out very deep recursive to avoid potential stack overflow in benchmark
    // group.bench_function("very_deep_recursive", |b| {
    //     b.iter(|| dfs_recursive(black_box(&very_deep_graph), black_box(0)))
    // });

    group.finish();
}

fn bench_shortest_path_small(c: &mut Criterion) {
    let graph = create_small_graph();
    c.bench_function("shortest_path_small", |b| {
        b.iter(|| shortest_path(black_box(&graph), black_box(0), black_box(9)))
    });
}

fn bench_shortest_path_medium(c: &mut Criterion) {
    let graph = create_medium_graph();
    c.bench_function("shortest_path_medium", |b| {
        b.iter(|| shortest_path(black_box(&graph), black_box(0), black_box(999)))
    });
}

fn bench_shortest_path_large(c: &mut Criterion) {
    let graph = create_large_graph();
    c.bench_function("shortest_path_large", |b| {
        b.iter(|| shortest_path(black_box(&graph), black_box(0), black_box(9999)))
    });
}

fn bench_cycle_detection_acyclic(c: &mut Criterion) {
    let graph = create_small_graph(); // Linear chain - no cycles
    c.bench_function("cycle_detection_acyclic", |b| {
        b.iter(|| has_cycle(black_box(&graph)))
    });
}

fn bench_cycle_detection_cyclic(c: &mut Criterion) {
    let graph = create_cyclic_graph(1000);
    c.bench_function("cycle_detection_cyclic", |b| {
        b.iter(|| has_cycle(black_box(&graph)))
    });
}

fn bench_find_cycle_large(c: &mut Criterion) {
    let graph = create_cyclic_graph(1000);
    c.bench_function("find_cycle_large", |b| {
        b.iter(|| find_cycle(black_box(&graph)))
    });
}

fn bench_connected_components_single(c: &mut Criterion) {
    let graph = create_medium_graph(); // Single connected component
    c.bench_function("connected_components_single", |b| {
        b.iter(|| connected_components(black_box(&graph)))
    });
}

fn bench_connected_components_multiple(c: &mut Criterion) {
    let graph = create_disconnected_graph(10, 100); // 10 components of 100 nodes each
    c.bench_function("connected_components_multiple", |b| {
        b.iter(|| connected_components(black_box(&graph)))
    });
}

fn bench_complete_graph_small(c: &mut Criterion) {
    let graph = create_complete_graph(50); // Complete graph is worst case
    c.bench_function("complete_graph_bfs_50", |b| {
        b.iter(|| bfs(black_box(&graph), black_box(0)))
    });
}

fn bench_complete_graph_dfs(c: &mut Criterion) {
    let graph = create_complete_graph(50);
    c.bench_function("complete_graph_dfs_50", |b| {
        b.iter(|| dfs(black_box(&graph), black_box(0)))
    });
}

criterion_group!(
    bfs_benchmarks,
    bench_bfs_small_graph,
    bench_bfs_medium_graph,
    bench_bfs_large_graph
);

criterion_group!(
    dfs_benchmarks,
    bench_dfs_small_graph,
    bench_dfs_medium_graph,
    bench_dfs_large_graph
);

criterion_group!(
    shortest_path_benchmarks,
    bench_shortest_path_small,
    bench_shortest_path_medium,
    bench_shortest_path_large
);

criterion_group!(
    cycle_detection_benchmarks,
    bench_cycle_detection_acyclic,
    bench_cycle_detection_cyclic,
    bench_find_cycle_large
);

criterion_group!(
    component_benchmarks,
    bench_connected_components_single,
    bench_connected_components_multiple
);

criterion_group!(
    worst_case_benchmarks,
    bench_complete_graph_small,
    bench_complete_graph_dfs
);

criterion_group!(
    recursive_vs_iterative_benchmarks,
    bench_dfs_iterative_vs_recursive_small,
    bench_dfs_iterative_vs_recursive_medium,
    bench_dfs_iterative_vs_recursive_large,
    bench_dfs_deep_graph_stack_safety
);

criterion_main!(
    bfs_benchmarks,
    dfs_benchmarks,
    shortest_path_benchmarks,
    cycle_detection_benchmarks,
    component_benchmarks,
    worst_case_benchmarks,
    recursive_vs_iterative_benchmarks
);
