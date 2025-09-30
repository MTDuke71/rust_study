use criterion::{criterion_group, criterion_main, Criterion};
use competitive_linked_tree::{TreeDiameter, Tree};
use std::hint::black_box;

fn bench_tree_creation(c: &mut Criterion) {
    c.bench_function("tree_creation_100", |b| {
        b.iter(|| {
            // Create line graph: 0-1-2-...-99
            let edges: Vec<_> = (0..99).map(|i| (i, i + 1)).collect();
            black_box(Tree::from_edges(100, edges))
        })
    });
}

fn bench_diameter_line_graph(c: &mut Criterion) {
    c.bench_function("diameter_line_graph_100", |b| {
        b.iter(|| {
            // Create line graph: 0-1-2-...-99 (diameter should be 99)
            let edges: Vec<_> = (0..99).map(|i| (i, i + 1)).collect();
            let diameter_finder = TreeDiameter::from_edges(100, edges).unwrap();
            black_box(diameter_finder.find_diameter())
        })
    });
}

fn bench_diameter_star_graph(c: &mut Criterion) {
    c.bench_function("diameter_star_graph_100", |b| {
        b.iter(|| {
            // Create star graph: 0 connected to all other nodes (diameter should be 2)
            let edges: Vec<_> = (1..100).map(|i| (0, i)).collect();
            let diameter_finder = TreeDiameter::from_edges(100, edges).unwrap();
            black_box(diameter_finder.find_diameter())
        })
    });
}

fn bench_diameter_binary_tree(c: &mut Criterion) {
    c.bench_function("diameter_binary_tree_127", |b| {
        b.iter(|| {
            // Create complete binary tree with 127 nodes (7 levels)
            let mut edges = Vec::new();
            for i in 0..63 {
                let left_child = 2 * i + 1;
                let right_child = 2 * i + 2;
                if left_child < 127 {
                    edges.push((i, left_child));
                }
                if right_child < 127 {
                    edges.push((i, right_child));
                }
            }
            let diameter_finder = TreeDiameter::from_edges(127, edges).unwrap();
            black_box(diameter_finder.find_diameter())
        })
    });
}

fn bench_large_tree_diameter(c: &mut Criterion) {
    c.bench_function("diameter_large_tree_1000", |b| {
        b.iter(|| {
            // Create a larger line graph for performance testing
            let edges: Vec<_> = (0..999).map(|i| (i, i + 1)).collect();
            let diameter_finder = TreeDiameter::from_edges(1000, edges).unwrap();
            black_box(diameter_finder.find_diameter())
        })
    });
}

criterion_group!(
    benches, 
    bench_tree_creation,
    bench_diameter_line_graph,
    bench_diameter_star_graph,
    bench_diameter_binary_tree,
    bench_large_tree_diameter
);
criterion_main!(benches);