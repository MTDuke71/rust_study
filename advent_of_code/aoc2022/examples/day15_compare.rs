use aoc2022::solver::day15;
use std::time::Instant;

const EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

fn main() {
    let sensors = day15::parse(EXAMPLE);

    println!("🎯 Day 15 Part 2 Performance Comparison (Example Input)");
    println!("{}", "=".repeat(60));
    println!();

    // Original approach
    let start = Instant::now();
    let result1 = day15::part2(&sensors);
    let time1 = start.elapsed();
    println!("✅ Original (row scan):     {:?} → {}", time1, result1);

    // HashMap pre-computation
    let start = Instant::now();
    let result2 = day15::part2_hashmap(&sensors, 20);
    let time2 = start.elapsed();
    println!("✅ HashMap pre-compute:     {:?} → {}", time2, result2);

    // Perimeter search
    let start = Instant::now();
    let result3 = day15::part2_perimeter(&sensors, 20);
    let time3 = start.elapsed();
    println!("✅ Perimeter search:        {:?} → {}", time3, result3);

    println!();
    println!("Speedup factors:");
    println!("  HashMap vs Original:    {:.2}×", time1.as_secs_f64() / time2.as_secs_f64());
    println!("  Perimeter vs Original:  {:.2}×", time1.as_secs_f64() / time3.as_secs_f64());
    println!();

    // Now with actual input
    println!();
    println!("🎯 Day 15 Part 2 Performance Comparison (Actual Input)");
    println!("{}", "=".repeat(60));
    println!();

    let input = include_str!("../inputs/day15.txt");
    let sensors = day15::parse(input);

    // Original approach
    println!("Running original...");
    let start = Instant::now();
    let result1 = day15::part2(&sensors);
    let time1 = start.elapsed();
    println!("✅ Original (row scan):     {:?} → {}", time1, result1);

    // HashMap pre-computation
    println!("Running HashMap...");
    let start = Instant::now();
    let result2 = day15::part2_hashmap(&sensors, 4_000_000);
    let time2 = start.elapsed();
    println!("✅ HashMap pre-compute:     {:?} → {}", time2, result2);

    // Perimeter search
    println!("Running perimeter...");
    let start = Instant::now();
    let result3 = day15::part2_perimeter(&sensors, 4_000_000);
    let time3 = start.elapsed();
    println!("✅ Perimeter search:        {:?} → {}", time3, result3);

    println!();
    println!("Speedup factors:");
    if time1.as_secs_f64() > time2.as_secs_f64() {
        println!("  HashMap vs Original:    {:.2}× FASTER", time1.as_secs_f64() / time2.as_secs_f64());
    } else {
        println!("  HashMap vs Original:    {:.2}× SLOWER", time2.as_secs_f64() / time1.as_secs_f64());
    }
    if time1.as_secs_f64() > time3.as_secs_f64() {
        println!("  Perimeter vs Original:  {:.2}× FASTER", time1.as_secs_f64() / time3.as_secs_f64());
    } else {
        println!("  Perimeter vs Original:  {:.2}× SLOWER", time3.as_secs_f64() / time1.as_secs_f64());
    }
}
