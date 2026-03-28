use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler::problems::{p001, p006, p007, p008, p009, p010, p011, p012, p013, p014, p017, p018, p019, p020, p021, p022, p023, p024, p067};

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
    c.bench_function("Problem 14", |b| b.iter(|| p014::solve()));
    c.bench_function("Problem 14 (naive)", |b| {
        b.iter(|| {
            (1u64..black_box(1_000_000))
                .max_by_key(|&n| p014::collatz_chain_length(n))
                .unwrap()
        })
    });
    c.bench_function("Problem 17", |b| b.iter(|| p017::solve()));
    c.bench_function("Problem 18 (DP)", |b| b.iter(|| p018::solve()));
    c.bench_function("Problem 18 (brute force)", |b| {
        let triangle = p018::parse_triangle(p018::TRIANGLE_INPUT);
        let flat = p018::flatten_triangle(&triangle);
        b.iter(|| p018::brute_force_max_path(black_box(&flat), 15))
    });
    c.bench_function("Problem 19", |b| b.iter(|| p019::solve()));
    c.bench_function("Problem 20", |b| b.iter(|| p020::solve()));
    c.bench_function("Problem 21", |b| b.iter(|| p021::solve()));
    c.bench_function("Problem 22", |b| b.iter(|| p022::solve()));
    c.bench_function("Problem 23", |b| b.iter(|| p023::solve()));
    c.bench_function("Problem 24", |b| b.iter(|| p024::solve()));
    c.bench_function("Problem 67", |b| b.iter(|| p067::solve()));
}

criterion_group!(benches, benchmark_problems);
criterion_main!(benches);
