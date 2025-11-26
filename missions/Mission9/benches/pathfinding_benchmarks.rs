use criterion::{criterion_group, criterion_main, Criterion};
use mission9::{
    AstarPathfinder, DijkstraPathfinder, EuclideanHeuristic, HeuristicContext, Pathfinder,
    SimpleWeightedGraph,
};
use std::hint::black_box;

fn create_grid_graph(size: usize) -> (SimpleWeightedGraph, HeuristicContext) {
    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // Create size x size grid
    for y in 0..size {
        for x in 0..size {
            let node = (y * size + x) as u32;
            context.add_coordinates(node, x as f64, y as f64);

            // Add horizontal connections
            if x < size - 1 {
                let right = (y * size + x + 1) as u32;
                graph.add_undirected_edge(node, right, 1.0);
            }

            // Add vertical connections
            if y < size - 1 {
                let below = ((y + 1) * size + x) as u32;
                graph.add_undirected_edge(node, below, 1.0);
            }
        }
    }

    (graph, context)
}

fn benchmark_dijkstra_small_grid(c: &mut Criterion) {
    let (graph, _) = create_grid_graph(10);
    let pathfinder = DijkstraPathfinder::new();

    c.bench_function("dijkstra_10x10_grid", |b| {
        b.iter(|| pathfinder.find_path(black_box(&graph), black_box(0), black_box(99)))
    });
}

fn benchmark_astar_small_grid(c: &mut Criterion) {
    let (graph, context) = create_grid_graph(10);
    let pathfinder = AstarPathfinder::new(EuclideanHeuristic);

    c.bench_function("astar_10x10_grid", |b| {
        b.iter(|| {
            pathfinder.find_path_with_context(
                black_box(&graph),
                black_box(0),
                black_box(99),
                black_box(&context),
            )
        })
    });
}

fn benchmark_comparison_medium_grid(c: &mut Criterion) {
    let (graph, context) = create_grid_graph(20);
    let dijkstra = DijkstraPathfinder::new();
    let astar = AstarPathfinder::new(EuclideanHeuristic);

    let mut group = c.benchmark_group("pathfinding_comparison_20x20");

    group.bench_function("dijkstra", |b| {
        b.iter(|| dijkstra.find_path(black_box(&graph), black_box(0), black_box(399)))
    });

    group.bench_function("astar", |b| {
        b.iter(|| {
            astar.find_path_with_context(
                black_box(&graph),
                black_box(0),
                black_box(399),
                black_box(&context),
            )
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_dijkstra_small_grid,
    benchmark_astar_small_grid,
    benchmark_comparison_medium_grid
);
criterion_main!(benches);
