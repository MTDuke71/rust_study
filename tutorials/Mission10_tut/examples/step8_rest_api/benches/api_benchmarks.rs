use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use step8_rest_api::state::AppState;
use std::sync::Arc;

fn bench_create_instance(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_instance");
    
    for size in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let state = Arc::new(AppState::new());
            b.iter(|| {
                state.create_instance(black_box(size))
            });
        });
    }
    
    group.finish();
}

fn bench_union_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("union");
    
    let state = Arc::new(AppState::new());
    let id = state.create_instance(1000);
    
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let _ = state.get_instance(id, |uf| {
                uf.union(black_box(0), black_box(1))
            });
        });
    });
    
    group.finish();
}

fn bench_find_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("find");
    
    let state = Arc::new(AppState::new());
    let id = state.create_instance(1000);
    
    // Create some unions first
    let _ = state.get_instance(id, |uf| {
        for i in 0..500 {
            let _ = uf.union(i, i + 1);
        }
        Ok::<(), String>(())
    });
    
    group.bench_function("after_unions", |b| {
        b.iter(|| {
            let _ = state.get_instance(id, |uf| {
                uf.find(black_box(250))
            });
        });
    });
    
    group.finish();
}

fn bench_connected_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("connected");
    
    let state = Arc::new(AppState::new());
    let id = state.create_instance(1000);
    
    group.bench_function("disconnected", |b| {
        b.iter(|| {
            let _ = state.get_instance(id, |uf| {
                uf.connected(black_box(0), black_box(999))
            });
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_create_instance,
    bench_union_operations,
    bench_find_operations,
    bench_connected_operations
);
criterion_main!(benches);
