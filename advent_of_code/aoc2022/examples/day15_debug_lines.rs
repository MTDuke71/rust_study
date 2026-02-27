use aoc2022::solver::day15;

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

    println!("🔍 Debugging line-based search");
    println!("Expected answer: (14, 11) → 56000011");
    println!();

    use std::collections::HashSet;

    let mut pos_lines = HashSet::new();
    let mut neg_lines = HashSet::new();

    for (i, sensor) in sensors.iter().enumerate() {
        let r = sensor.radius() + 1;
        let cx = sensor.x;
        let cy = sensor.y;

        let pos1 = cy - cx + r;
        let pos2 = cy - cx - r;
        let neg1 = cy + cx + r;
        let neg2 = cy + cx - r;

        println!("Sensor {}: ({}, {}) r={}", i, cx, cy, r - 1);
        println!("  Pos lines: y-x={}, y-x={}", pos1, pos2);
        println!("  Neg lines: y+x={}, y+x={}", neg1, neg2);

        pos_lines.insert(pos1);
        pos_lines.insert(pos2);
        neg_lines.insert(neg1);
        neg_lines.insert(neg2);
    }

    println!();
    println!("Total unique pos lines: {}", pos_lines.len());
    println!("Total unique neg lines: {}", neg_lines.len());

    // Find pairs with 2-unit gaps
    let mut pos_pairs = Vec::new();
    for &c in &pos_lines {
        if pos_lines.contains(&(c + 2)) {
            pos_pairs.push((c, c + 2));
        }
    }

    let mut neg_pairs = Vec::new();
    for &c in &neg_lines {
        if neg_lines.contains(&(c + 2)) {
            neg_pairs.push((c, c + 2));
        }
    }

    println!();
    println!("Pos line pairs (2 apart): {:?}", pos_pairs);
    println!("Neg line pairs (2 apart): {:?}", neg_pairs);

    // Check expected answer
    let x = 14;
    let y = 11;
    println!();
    println!(
        "Expected point ({}, {}): y-x={}, y+x={}",
        x,
        y,
        y - x,
        y + x
    );

    // See which lines pass through this point
    println!("Lines through expected point:");
    if pos_lines.contains(&(y - x)) {
        println!("  y-x={} ✓", y - x);
    }
    if pos_lines.contains(&(y - x - 1)) && pos_lines.contains(&(y - x + 1)) {
        println!("  Between y-x={} and y-x={}", y - x - 1, y - x + 1);
    }
    if neg_lines.contains(&(y + x)) {
        println!("  y+x={} ✓", y + x);
    }
    if neg_lines.contains(&(y + x - 1)) && neg_lines.contains(&(y + x + 1)) {
        println!("  Between y+x={} and y+x={}", y + x - 1, y + x + 1);
    }
}
