//! Alternative Day 10 Part 2: Using Shoelace Formula + Pick's Theorem
//!
//! ## Mathematical Approach
//!
//! Instead of ray casting to count interior points, we can use the same
//! mathematical approach as Day 18:
//!
//! 1. **Trace the loop** to get ordered vertices
//! 2. **Shoelace Formula** to compute interior area
//! 3. **Pick's Theorem** to find interior lattice points
//!
//! ### Why This Works
//!
//! The pipe loop forms a **polygon on an integer lattice**:
//! - Each pipe segment connects adjacent grid cells
//! - Vertices are at integer coordinates
//! - The loop encloses a region
//!
//! ### Formula
//!
//! Given a polygon with vertices and perimeter:
//! - **Shoelace**: Computes area A from vertices
//! - **Pick's Theorem**: A = I + B/2 - 1
//! - Rearranged: **I = A - B/2 + 1**
//!
//! Where:
//! - A = interior area (from Shoelace)
//! - I = interior lattice points (what we want)
//! - B = boundary points (loop length)
//!
//! ### Comparison with Original
//!
//! **Ray Casting** (original):
//! - Scans every grid cell: O(width × height)
//! - Complex corner state machine
//! - Handles Jordan Curve Theorem crossings
//!
//! **Shoelace + Pick** (this version):
//! - Only processes loop vertices: O(loop_length)
//! - Pure arithmetic on coordinates
//! - Elegant mathematical solution
//!
//! Both give the same answer, but this approach is:
//! - Faster for large grids with small loops
//! - More elegant mathematically
//! - Same algorithm as Day 18!

use aoc2023::solver::day10::{find_loop_vertices, parse_grid, find_start};
use anyhow::Result;

fn main() -> Result<()> {
    // Load the actual puzzle input
    let input = include_str!("../inputs/day10.txt");
    
    println!("Day 10 Part 2: Shoelace Formula + Pick's Theorem");
    println!("{}", "=".repeat(60));
    println!();
    
    // Parse grid and find the loop
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find 'S'");
    let vertices = find_loop_vertices(&grid, start);
    
    println!("Loop Statistics:");
    println!("  Total vertices: {}", vertices.len());
    println!("  Perimeter (B):  {} cells", vertices.len());
    println!();
    
    // Apply Shoelace formula to get interior area
    let area = shoelace_area(&vertices);
    println!("Shoelace Formula:");
    println!("  Interior area:  {:.1} square units", area);
    println!();
    
    // Apply Pick's theorem: A = I + B/2 - 1
    // Rearranged: I = A - B/2 + 1
    let perimeter = vertices.len() as f64;
    let interior_points = area - perimeter / 2.0 + 1.0;
    
    println!("Pick's Theorem (A = I + B/2 - 1):");
    println!("  A = {:.1}", area);
    println!("  B = {}", perimeter as i64);
    println!("  Solving for I:");
    println!("  I = A - B/2 + 1");
    println!("  I = {:.1} - {:.1} + 1", area, perimeter / 2.0);
    println!("  I = {}", interior_points as i64);
    println!();
    
    println!("✅ Interior points enclosed by loop: {}", interior_points as i64);
    
    Ok(())
}

/// Shoelace formula to compute polygon area
///
/// Area = (1/2) |Σ(x_i × y_{i+1} - x_{i+1} × y_i)|
///
/// For a polygon traced as a closed loop, this gives the interior area.
fn shoelace_area(vertices: &[(i64, i64)]) -> f64 {
    let n = vertices.len();
    let mut sum = 0i64;
    
    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];
        sum += x1 * y2 - x2 * y1;
    }
    
    (sum.abs() as f64) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_shoelace_example1() {
        // Simple 3×3 loop
        let grid = parse_grid(EXAMPLE1);
        let start = find_start(&grid).unwrap();
        let vertices = find_loop_vertices(&grid, start);
        
        let area = shoelace_area(&vertices);
        let interior = area - (vertices.len() as f64) / 2.0 + 1.0;
        
        assert_eq!(interior as i64, 1);
    }

    #[test]
    fn test_shoelace_example2() {
        let grid = parse_grid(EXAMPLE2);
        let start = find_start(&grid).unwrap();
        let vertices = find_loop_vertices(&grid, start);
        
        let area = shoelace_area(&vertices);
        let interior = area - (vertices.len() as f64) / 2.0 + 1.0;
        
        assert_eq!(interior as i64, 1);
    }

    #[test]
    fn test_shoelace_example3() {
        // Example with 4 interior points
        let grid = parse_grid(EXAMPLE3);
        let start = find_start(&grid).unwrap();
        let vertices = find_loop_vertices(&grid, start);
        
        let area = shoelace_area(&vertices);
        let interior = area - (vertices.len() as f64) / 2.0 + 1.0;
        
        assert_eq!(interior as i64, 4);
    }

    #[test]
    fn test_shoelace_example4() {
        // Example with 8 interior points
        let grid = parse_grid(EXAMPLE4);
        let start = find_start(&grid).unwrap();
        let vertices = find_loop_vertices(&grid, start);
        
        let area = shoelace_area(&vertices);
        let interior = area - (vertices.len() as f64) / 2.0 + 1.0;
        
        assert_eq!(interior as i64, 8);
    }

    #[test]
    fn test_shoelace_example5() {
        // Example with 10 interior points
        let grid = parse_grid(EXAMPLE5);
        let start = find_start(&grid).unwrap();
        let vertices = find_loop_vertices(&grid, start);
        
        let area = shoelace_area(&vertices);
        let interior = area - (vertices.len() as f64) / 2.0 + 1.0;
        
        assert_eq!(interior as i64, 10);
    }
}
