/// Day 21: Performance comparison between quadratic extrapolation and geometric counting
///
/// **BOTH METHODS NOW PRODUCE CORRECT RESULTS!**
///
/// This example demonstrates two fundamentally different optimization approaches
/// for solving the infinite grid reachability problem.
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
    println!("  ✓ Works for ANY input (general-purpose)");
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
    println!("  ⚠ Only works for symmetric grids (empty cross + borders)");
    println!();
    
    let start2 = Instant::now();
    let result2 = part2_optimized(&input);
    let duration2 = start2.elapsed();
    
    println!("  Result: {}", result2);
    println!("  Time:   {:?}", duration2);
    println!();
    
    // Comparison
    println!("=== Comparison ===");
    
    if result1 != result2 {
        println!("  ❌ MISMATCH!");
        println!("  Extrapolation: {}", result1);
        println!("  Geometric:     {}", result2);
        println!("  Difference:    {}", (result1 as i64 - result2 as i64).abs());
    } else {
        println!("  ✅ Both methods produce IDENTICAL results!");
        println!("  Answer: {}", result1);
    }
    
    let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
    println!("  Speedup: {:.2}× faster (geometric vs extrapolation)", speedup);
    println!();
    
    // Analysis
    println!("=== Trade-off Analysis ===");
    println!();
    println!("Quadratic Extrapolation ({:?}):", duration1);
    println!("  ✓ General-purpose - works for ANY input");
    println!("  ✓ Simpler logic - mathematical extrapolation");
    println!("  ✓ Better for teaching Lagrange interpolation");
    println!("  ✓ Proven correct approach");
    println!("  ✗ Slower (~{}ms)", duration1.as_millis());
    println!("  ✗ Uses floating-point arithmetic");
    println!();
    println!("Geometric Counting ({:?}):", duration2);
    println!("  ✓ {:.2}× faster!", speedup);
    println!("  ✓ Exact integer arithmetic (no rounding errors)");
    println!("  ✓ Optimal for competitive programming");
    println!("  ✓ Demonstrates geometric reasoning");
    println!("  ✗ Complex tile classification logic");
    println!("  ✗ ONLY works for symmetric grids:");
    println!("      - Empty cardinal cross through start");
    println!("      - Empty borders");
    println!("      - Odd grid size with center start");
    println!("  ✗ Breaks on adversarial/irregular inputs");
    println!();
    println!("=== Recommendation ===");
    println!("- Use quadratic extrapolation for: General solutions, learning, production");
    println!("- Use geometric counting for: Speed competitions, symmetric puzzle inputs");
}
