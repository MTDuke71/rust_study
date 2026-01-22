/// Day 21: Performance comparison between quadratic extrapolation and geometric counting
///
/// Run with: cargo run --release --example day21_comparison

use aoc2023::solver::day21::{part2, part2_optimized};
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("inputs/day21.txt")
        .expect("Failed to read day21.txt");
    
    println!("=== Day 21: Part 2 Method Comparison ===\n");
    
    // Method 1: Quadratic Extrapolation (General Solution)
    println!("Method 1: Quadratic Extrapolation via Lagrange Interpolation");
    println!("  - Samples 3 points (65, 196, 327 steps)");
    println!("  - Fits quadratic: f(n) = an² + bn + c");
    println!("  - Extrapolates to n=202,300");
    println!();
    
    let start1 = Instant::now();
    let result1 = part2(&input);
    let duration1 = start1.elapsed();
    
    println!("  Result: {}", result1);
    println!("  Time:   {:?}", duration1);
    println!();
    
    // Method 2: Geometric Counting (Optimized for Symmetric Grids)
    println!("Method 2: Direct Geometric Counting (Exploits Grid Symmetry)");
    println!("  - Classifies tiles by entry point and parity");
    println!("  - Runs 13 BFS (2 full + 4 corners + 4 small + 4 large edges)");
    println!("  - Counts tiles geometrically in diamond pattern");
    println!();
    
    let start2 = Instant::now();
    let result2 = part2_optimized(&input);
    let duration2 = start2.elapsed();
    
    println!("  Result: {}", result2);
    println!("  Time:   {:?}", duration2);
    println!();
    
    // Comparison
    println!("=== Comparison ===");
    println!("  Results match: {}", result1 == result2);
    
    if result1 != result2 {
        println!("  ❌ MISMATCH!");
        println!("  Extrapolation: {}", result1);
        println!("  Geometric:     {}", result2);
        println!("  Difference:    {}", (result1 as i64 - result2 as i64).abs());
    } else {
        println!("  ✓ Both produce: {}", result1);
    }
    
    let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
    println!("  Speedup: {:.2}× faster", speedup);
    println!();
    
    // Analysis
    println!("=== Trade-off Analysis ===");
    println!("Quadratic Extrapolation:");
    println!("  ✓ General-purpose (works for any input)");
    println!("  ✓ Simpler logic (mathematical extrapolation)");
    println!("  ✓ Better for teaching Lagrange interpolation");
    println!("  ✗ Slower (~{:?})", duration1);
    println!("  ✗ Uses floating-point arithmetic (potential precision issues)");
    println!();
    println!("Geometric Counting:");
    println!("  ✓ Much faster (~{:?})", duration2);
    println!("  ✓ Exact integer arithmetic (no rounding)");
    println!("  ✓ Better for competitive programming");
    println!("  ✗ Complex tile classification logic");
    println!("  ✗ Only works for symmetric grids (empty rows/cols/borders)");
    println!("  ✗ Breaks on adversarial inputs");
}
