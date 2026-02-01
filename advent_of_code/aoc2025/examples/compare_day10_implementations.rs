/// Compare day10.rs (good_lp) and day10_alt.rs (Z3) implementations
///
/// This example runs both implementations on the actual puzzle input
/// and verifies they produce identical results.
use aoc2025::solver::{day10, day10_alt};
use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("inputs/day10.txt")?;

    println!("🔍 Comparing Day 10 implementations...\n");

    // Part 1 - Both use identical Gaussian elimination
    println!("Part 1 (Gaussian Elimination over GF(2)):");
    let p1_goodlp = day10::solve_part1(&input)?;
    let p1_z3 = day10_alt::solve_part1(&input)?;

    println!("  good_lp (day10.rs): {}", p1_goodlp);
    println!("  Z3 (day10_alt.rs):  {}", p1_z3);

    if p1_goodlp == p1_z3 {
        println!("  ✅ Part 1 results MATCH!\n");
    } else {
        println!("  ❌ Part 1 results DIFFER!\n");
        return Err(anyhow::anyhow!(
            "Part 1 mismatch: {} vs {}",
            p1_goodlp,
            p1_z3
        ));
    }

    // Part 2 - Different solvers (good_lp ILP vs Z3 SMT)
    println!("Part 2 (Integer Linear Programming):");
    let p2_goodlp = day10::solve_part2(&input)?;
    let p2_z3 = day10_alt::solve_part2(&input)?;

    println!("  good_lp/minilp (day10.rs): {}", p2_goodlp);
    println!("  Z3 SMT solver (day10_alt.rs): {}", p2_z3);

    if p2_goodlp == p2_z3 {
        println!("  ✅ Part 2 results MATCH!\n");
    } else {
        println!("  ❌ Part 2 results DIFFER!\n");
        return Err(anyhow::anyhow!(
            "Part 2 mismatch: {} vs {}",
            p2_goodlp,
            p2_z3
        ));
    }

    println!("🎉 SUCCESS! Both implementations produce identical results:");
    println!("   Part 1: {}", p1_goodlp);
    println!("   Part 2: {}", p2_goodlp);

    Ok(())
}
