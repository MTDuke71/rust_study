use criterion::{black_box, criterion_group, criterion_main, Criterion};
use competitive_linked_tree::Tree;

fn bench_tree_creation(c: &mut Criterion) {
    c.bench_function("tree_creation_100", |b| {
        b.iter(|| {
            // Create line graph: 0-1-2-...-99
            let edges: Vec<_> = (0..99).map(|i| (i, i + 1)).collect();
            black_box(Tree::from_edges(100, edges))
        })
    });
}

criterion_group!(benches, bench_tree_creation);
criterion_main!(benches);