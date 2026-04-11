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

fn day05_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day05.txt").expect("Need day05.txt input");
    c.bench_function("day05", |b| b.iter(|| aoc2017::solver::day05::solve(&input)));
}

fn day06_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day06.txt").expect("Need day06.txt input");
    c.bench_function("day06", |b| b.iter(|| aoc2017::solver::day06::solve(&input)));
}

fn day07_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day07.txt").expect("Need day07.txt input");
    c.bench_function("day07", |b| b.iter(|| aoc2017::solver::day07::solve(&input)));
}

fn day08_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day08.txt").expect("Need day08.txt input");
    c.bench_function("day08", |b| b.iter(|| aoc2017::solver::day08::solve(&input)));
}

fn day09_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day09.txt").expect("Need day09.txt input");
    c.bench_function("day09", |b| b.iter(|| aoc2017::solver::day09::solve(&input)));
}

criterion_group!(benches, day01_bench, day02_bench, day03_bench, day04_bench, day05_bench, day06_bench, day07_bench, day08_bench, day09_bench);
criterion_main!(benches);
