use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use project_euler::problems::*;

fn benchmark_problems(_c: &mut Criterion) {
    // Benchmarks will be added as problems are implemented
    // Example:
    // c.bench_function("Problem 1", |b| b.iter(|| p001::solve()));
}

criterion_group!(benches, benchmark_problems);
criterion_main!(benches);
