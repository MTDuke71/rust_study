use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::problems::{p001, p006, p007, p008, p009, p010, p011, p012, p013};

fn benchmark_problems(c: &mut Criterion) {
    c.bench_function("Problem 1", |b| b.iter(|| p001::solve()));
    c.bench_function("Problem 6", |b| b.iter(|| p006::solve()));
    c.bench_function("Problem 7", |b| {
        b.iter(|| p007::nth_prime(black_box(10001)))
    });
    c.bench_function("Problem 8", |b| b.iter(|| p008::solve()));
    c.bench_function("Problem 9", |b| b.iter(|| p009::solve()));
    c.bench_function("Problem 10", |b| b.iter(|| p010::solve()));
    c.bench_function("Problem 11", |b| b.iter(|| p011::solve()));
    c.bench_function("Problem 12", |b| b.iter(|| p012::solve()));
    c.bench_function("Problem 13", |b| b.iter(|| p013::solve()));
    c.bench_function("Problem 13 (u128 15-digit)", |b| {
        b.iter(|| p013::first_ten_digits_of_sum())
    });
    c.bench_function("Problem 13 (u128 12-digit)", |b| {
        b.iter(|| p013::first_ten_digits_of_sum_12())
    });
    c.bench_function("Problem 13 (f64)", |b| {
        b.iter(|| p013::first_ten_digits_of_sum_f64())
    });
}

criterion_group!(benches, benchmark_problems);
criterion_main!(benches);
