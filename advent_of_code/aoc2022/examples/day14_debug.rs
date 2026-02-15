//! Debug why position (4, 8) isn't filled in Part 1
//!
//! This will manually simulate and show grid state to understand
//! why that position appears empty.

use aoc2022::solver::day14;

const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

fn main() {
    let data = day14::parse(EXAMPLE);

    println!("Running Part 1 simulation...");
    let count = day14::part1(&data);
    println!("Result: {} grains\n", count);

    // Now let's manually examine the rock structure
    println!("Original rock paths:");
    println!("  Path 1: 498,4 -> 498,6 -> 496,6");
    println!("  Path 2: 503,4 -> 502,4 -> 502,9 -> 494,9");
    println!();

    // The issue: in the visual output, position (4, 8) appears empty
    // but has a rock below it at (4, 9)
    //
    // Let's think about the rock structure:
    // Path 1 creates: vertical line (498,4)-(498,6) and horizontal (498,6)-(496,6)
    // Path 2 creates: H(503,4)-(502,4), V(502,4)-(502,9), H(502,9)-(494,9)
    //
    // After normalization (x_offset = 492), this becomes:
    // Path 1: V(6,4)-(6,6), H(6,6)-(4,6)
    // Path 2: H(11,4)-(10,4), V(10,4)-(10,9), H(10,9)-(2,9)
    //
    // So at grid position (4, 6) there IS a rock (from Path 1 horizontal line)
    // And at (4, 9) there IS a rock (from Path 2 horizontal line)
    //
    // The question: why doesn't sand fill (4, 7) or (4, 8)?

    println!("Rock structure after normalization:");
    println!("  At (4, 6): Rock (from Path 1)");
    println!("  At (4, 7): Air");
    println!("  At (4, 8): Air (but why no sand?)");
    println!("  At (4, 9): Rock (from Path 2)");
    println!();

    println!("Hypothesis: Sand cannot reach (4, 7) or (4, 8) due to blocking rocks.");
    println!();
    println!("Let's trace paths from source (8, 0):");
    println!("  Sand can move: down, down-left, down-right");
    println!("  To reach x=4 from x=8 requires 4 diagonal-left moves");
    println!("  But there's a rock wall at (4-6, 6) blocking leftward movement");
    println!();
    println!("When sand falls and hits the rock at (4, 6),");
    println!("  it can't moveinto (4, 7) from above (blocked by rock at 4,6)");
    println!("  it can't come from right (blocked by rocks and sand)");
    println!();
    println!("Conclusion: Position (4, 7) and (4, 8) are UNREACHABLE");
    println!("due to the rock at (4, 6) above them!");
}
