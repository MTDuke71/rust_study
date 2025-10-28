// Mission 10 Performance Benchmarks
// Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mission10::UnionFind;

fn bench_find_operations(c: &mut Criterion) {
    c.bench_function("find operations", |b| {
        let mut uf = UnionFind::new(1000);
        // Create some unions first
        for i in 0..999 {
            uf.union(i, i + 1).unwrap();
        }
        
        b.iter(|| {
            for i in 0..1000 {
                black_box(uf.find(i).unwrap());
            }
        });
    });
}

fn bench_union_operations(c: &mut Criterion) {
    c.bench_function("union operations", |b| {
        b.iter(|| {
            let mut uf = UnionFind::new(1000);
            for i in 0..999 {
                black_box(uf.union(i, i + 1).unwrap());
            }
        });
    });
}

fn bench_connected_queries(c: &mut Criterion) {
    c.bench_function("connected queries", |b| {
        let mut uf = UnionFind::new(1000);
        // Create some unions
        for i in 0..500 {
            uf.union(i, i + 1).unwrap();
        }
        
        b.iter(|| {
            for i in 0..500 {
                black_box(uf.connected(i, i + 1).unwrap());
            }
        });
    });
}

criterion_group!(benches, bench_find_operations, bench_union_operations, bench_connected_queries);
criterion_main!(benches);
