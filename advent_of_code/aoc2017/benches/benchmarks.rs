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

fn day10_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day10.txt").expect("Need day10.txt input");
    c.bench_function("day10", |b| b.iter(|| aoc2017::solver::day10::solve(&input)));
}

fn day11_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day11.txt").expect("Need day11.txt input");
    c.bench_function("day11", |b| b.iter(|| aoc2017::solver::day11::solve(&input)));
}

fn day12_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day12.txt").expect("Need day12.txt input");
    c.bench_function("day12", |b| b.iter(|| aoc2017::solver::day12::solve(&input)));
}

fn day13_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day13.txt").expect("Need day13.txt input");
    c.bench_function("day13", |b| b.iter(|| aoc2017::solver::day13::solve(&input)));
}

fn day14_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day14.txt").expect("Need day14.txt input");
    c.bench_function("day14", |b| b.iter(|| aoc2017::solver::day14::solve(&input)));
}

fn day15_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day15.txt").expect("Need day15.txt input");
    c.bench_function("day15", |b| b.iter(|| aoc2017::solver::day15::solve(&input)));
}

fn day16_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day16.txt").expect("Need day16.txt input");
    c.bench_function("day16", |b| b.iter(|| aoc2017::solver::day16::solve(&input)));
}

fn day17_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day17.txt").expect("Need day17.txt input");
    c.bench_function("day17", |b| b.iter(|| aoc2017::solver::day17::solve(&input)));
}

fn day18_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day18.txt").expect("Need day18.txt input");
    c.bench_function("day18", |b| b.iter(|| aoc2017::solver::day18::solve(&input)));
    c.bench_function("day18_part1", |b| b.iter(|| aoc2017::solver::day18::solve_part1(&input)));
    c.bench_function("day18_part2", |b| b.iter(|| aoc2017::solver::day18::solve_part2(&input)));
}

fn day19_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day19.txt").expect("Need day19.txt input");
    c.bench_function("day19", |b| b.iter(|| aoc2017::solver::day19::solve(&input)));
    c.bench_function("day19_part1", |b| b.iter(|| aoc2017::solver::day19::solve_part1(&input)));
    c.bench_function("day19_part2", |b| b.iter(|| aoc2017::solver::day19::solve_part2(&input)));
}

fn day20_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day20.txt").expect("Need day20.txt input");
    c.bench_function("day20", |b| b.iter(|| aoc2017::solver::day20::solve(&input)));
    c.bench_function("day20_part1", |b| b.iter(|| aoc2017::solver::day20::solve_part1(&input)));
    c.bench_function("day20_part2", |b| b.iter(|| aoc2017::solver::day20::solve_part2(&input)));
}

fn day21_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day21.txt").expect("Need day21.txt input");
    c.bench_function("day21", |b| b.iter(|| aoc2017::solver::day21::solve(&input)));
    c.bench_function("day21_part1", |b| b.iter(|| aoc2017::solver::day21::solve_part1(&input)));
    c.bench_function("day21_part2", |b| b.iter(|| aoc2017::solver::day21::solve_part2(&input)));
}

fn day21_bitpacked_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day21.txt").expect("Need day21.txt input");
    c.bench_function("day21_bitpacked", |b| b.iter(|| aoc2017::solver::day21_bitpacked::solve(&input)));
    c.bench_function("day21_bitpacked_part1", |b| b.iter(|| aoc2017::solver::day21_bitpacked::solve_part1(&input)));
    c.bench_function("day21_bitpacked_part2", |b| b.iter(|| aoc2017::solver::day21_bitpacked::solve_part2(&input)));
}

fn day21_memo_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day21.txt").expect("Need day21.txt input");
    c.bench_function("day21_memo", |b| b.iter(|| aoc2017::solver::day21_memo::solve(&input)));
    c.bench_function("day21_memo_part1", |b| b.iter(|| aoc2017::solver::day21_memo::solve_part1(&input)));
    c.bench_function("day21_memo_part2", |b| b.iter(|| aoc2017::solver::day21_memo::solve_part2(&input)));
}

fn day22_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day22.txt").expect("Need day22.txt input");
    c.bench_function("day22", |b| b.iter(|| aoc2017::solver::day22::solve(&input)));
    c.bench_function("day22_part1", |b| b.iter(|| aoc2017::solver::day22::solve_part1(&input)));
    c.bench_function("day22_part2", |b| b.iter(|| aoc2017::solver::day22::solve_part2(&input)));
}

fn day23_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day23.txt").expect("Need day23.txt input");
    c.bench_function("day23", |b| b.iter(|| aoc2017::solver::day23::solve(&input)));
    c.bench_function("day23_part1", |b| b.iter(|| aoc2017::solver::day23::solve_part1(&input)));
    c.bench_function("day23_part2", |b| b.iter(|| aoc2017::solver::day23::solve_part2(&input)));
}

fn day24_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day24.txt").expect("Need day24.txt input");
    c.bench_function("day24", |b| b.iter(|| aoc2017::solver::day24::solve(&input)));
    c.bench_function("day24_part1", |b| b.iter(|| aoc2017::solver::day24::solve_part1(&input)));
    c.bench_function("day24_part2", |b| b.iter(|| aoc2017::solver::day24::solve_part2(&input)));
}

fn day25_bench(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day25.txt").expect("Need day25.txt input");
    c.bench_function("day25", |b| b.iter(|| aoc2017::solver::day25::solve(&input)));
    c.bench_function("day25_part1", |b| b.iter(|| aoc2017::solver::day25::solve_part1(&input)));
}

criterion_group!(benches, day01_bench, day02_bench, day03_bench, day04_bench, day05_bench, day06_bench, day07_bench, day08_bench, day09_bench, day10_bench, day11_bench, day12_bench, day13_bench, day14_bench, day15_bench, day16_bench, day17_bench, day18_bench, day19_bench, day20_bench, day21_bench, day21_bitpacked_bench, day21_memo_bench, day22_bench, day23_bench, day24_bench, day25_bench);
criterion_main!(benches);
