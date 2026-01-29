use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::problems::{p001, p006, p007, p008, p009};

fn benchmark_problems(c: &mut Criterion) {
    c.bench_function("Problem 1", |b| b.iter(|| p001::solve()));
    c.bench_function("Problem 6", |b| b.iter(|| p006::solve()));
    c.bench_function("Problem 7", |b| {
        b.iter(|| p007::nth_prime(black_box(10001)))
    });
    c.bench_function("Problem 8", |b| b.iter(|| p008::solve()));
    c.bench_function("Problem 9", |b| b.iter(|| p009::solve()));
}

criterion_group!(benches, benchmark_problems);
criterion_main!(benches);
