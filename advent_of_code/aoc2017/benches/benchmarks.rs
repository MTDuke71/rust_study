use criterion::{criterion_group, criterion_main, Criterion};

fn day01_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day01.txt").expect("Need day01.txt input");
    c.bench_function("day01", |b| b.iter(|| aoc2017::solver::day01::solve(&input)));
}

fn day02_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day02.txt").expect("Need day02.txt input");
    c.bench_function("day02", |b| b.iter(|| aoc2017::solver::day02::solve(&input)));
}

fn day03_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day03.txt").expect("Need day03.txt input");
    c.bench_function("day03", |b| b.iter(|| aoc2017::solver::day03::solve(&input)));
}

fn day04_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day04.txt").expect("Need day04.txt input");
    c.bench_function("day04", |b| b.iter(|| aoc2017::solver::day04::solve(&input)));
}

criterion_group!(benches, day01_bench, day02_bench, day03_bench, day04_bench);
criterion_main!(benches);
