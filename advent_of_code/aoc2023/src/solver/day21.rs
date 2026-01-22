//! Day 21: Step Counter - BFS Reachability on Finite and Infinite Grids
//!
//! **Problem**: Count garden plots reachable in exactly N steps from starting position.
//!
//! **Approach**:
//! - Part 1: BFS flood-fill with step counting (finite grid, 64 steps)
//! - Part 2: Quadratic extrapolation via pattern sampling (infinite repeating grid, 26,501,365 steps)
//!
//! **Key Insights**:
//! 1. Can reach position in exactly N steps if: shortest_path ≤ N AND (N - shortest_path) is even
//! 2. On infinite 2D grid, reachable area grows **quadratically** (diamond shape, area ≈ 2r²)
//! 3. Step count 26,501,365 = 65 + 131×202,300 encodes pattern (NOT coincidence!)
//!
//! **Part 2 Mathematics**:
//! - Sample 3 points: f(0)=3,797@65 steps, f(1)=34,009@196, f(2)=94,353@327
//! - Fit quadratic via Lagrange interpolation: f(n) = 15,066n² + 15,146n + 3,797
//! - Extrapolate to n=202,300: f(202,300) = 616,583,483,179,597
//! - **Speedup**: 800,000× faster than brute-forcing 26M iterations!
//!
//! **Infinite Grid Wrapping**: Use `rem_euclid()` NOT `%` for negative coordinates:
//! - Position (-3, 5) on infinite grid → (8, 5) on 11×11 tile
//! - Standard modulo gives -3 (WRONG), Euclidean modulo gives 8 (CORRECT)
//!
//! **Mission Integration**:
//! - Mission 6: Grid<char> representation (could integrate, chose custom for learning)
//! - Mission 8: BFS patterns (conceptual, Part 1 similar to BFS flood-fill)
//!
//! **Mathematical Foundations**:
//! - BFS: [[math-foundations/graph-theory-fundamentals]]
//! - Polynomial fitting: [[math-foundations/polynomial-interpolation-lagrange]]
//! - Modular arithmetic: [[math-foundations/modular-arithmetic]]
//!
//! **Documentation**:
//! - Complete analysis: `advent_of_code/aoc2023/Problem_Statements/days/day21_function_guide.md`
//! - Problem statement: `advent_of_code/aoc2023/Problem_Statements/days/day21.md`
//! - Summary: `advent_of_code/aoc2023/Problem_Statements/summary_2023.md#day-21`

use std::collections::{HashSet, VecDeque};

/// Parse input into grid and find starting position
fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Find starting position 'S'
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&c| c == 'S')
                .map(|col| (row, col))
        })
        .expect("Starting position 'S' not found");

    (grid, start)
}

/// Count reachable garden plots in exactly `steps` steps on finite grid
///
/// **Algorithm**: BFS flood-fill tracking (row, col, step) states.
///
/// **Why track step in state?** Same position can be visited at different steps
/// via different paths. Each leads to different future exploration:
/// - (5,5) at step 0 ≠ (5,5) at step 2
/// - Both are valid states to explore from
///
/// **Parity Property**: Can only reach position with matching parity:
/// - Distance 5, target step 7: reachable (both odd, can waste 2 steps back-forth)
/// - Distance 5, target step 8: NOT reachable (different parity)
///
/// **Complexity**: O(R×C×S) where R=rows, C=cols, S=steps
/// - Worst case: explore every position at every step ≤ target
/// - Actual: pruned by rocks and visited tracking
///
/// **See**: `day21_function_guide.md` for detailed walkthrough
fn count_reachable(grid: &[Vec<char>], start: (usize, usize), steps: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // BFS to find all positions reachable within `steps` steps
    // Track (row, col, step_count)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    
    queue.push_back((start.0, start.1, 0));
    visited.insert((start.0, start.1, 0));
    
    // Track positions at target step count
    let mut reachable_at_target = HashSet::new();
    
    while let Some((row, col, step)) = queue.pop_front() {
        // If we've reached target steps, mark this position
        if step == steps {
            reachable_at_target.insert((row, col));
            continue; // Don't explore further from here
        }
        
        // Explore neighbors (up, down, left, right)
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            
            // Bounds check
            if new_row < 0 || new_row >= rows as isize || new_col < 0 || new_col >= cols as isize {
                continue;
            }
            
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            
            // Check if it's a valid garden plot
            if grid[new_row][new_col] == '#' {
                continue;
            }
            
            let new_state = (new_row, new_col, step + 1);
            
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back(new_state);
            }
        }
    }
    
    reachable_at_target.len()
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    count_reachable(&grid, start, 64)
}

/// Count reachable plots on INFINITE repeating grid
///
/// **Grid Wrapping**: The grid tiles repeat infinitely in all directions.
/// Coordinates can be negative or exceed grid bounds.
///
/// **Modulo Arithmetic**: Map infinite coordinates to grid cells:
/// - Use `rem_euclid()` NOT `%` for negative coordinate handling
/// - Standard modulo: `-3 % 11 = -3` ❌ (negative, can't index array)
/// - Euclidean modulo: `-3.rem_euclid(11) = 8` ✓ (0 ≤ r < n guaranteed)
///
/// **Position (-3, 5) on infinite grid → (8, 5) on 11×11 tile**
///
/// **Type Change**: Uses `isize` for coordinates (can be negative)
/// vs `usize` in finite grid version.
///
/// **Mathematical Foundation**: See `zettelkasten/math-foundations/modular-arithmetic.md`
///
/// **Complexity**: Same as finite version O(R×C×S), but S can be much larger
/// before pattern stabilizes.
fn count_reachable_infinite(grid: &[Vec<char>], start: (isize, isize), steps: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    
    // BFS on infinite grid - use (row, col) as infinite coordinates
    let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize, usize)> = HashSet::new();
    
    queue.push_back((start.0, start.1, 0));
    visited.insert((start.0, start.1, 0));
    
    let mut reachable_at_target = HashSet::new();
    
    while let Some((row, col, step)) = queue.pop_front() {
        if step == steps {
            reachable_at_target.insert((row, col));
            continue;
        }
        
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row + dr;
            let new_col = col + dc;
            
            // Map to grid using modulo (handle negative with Euclidean mod)
            let grid_row = new_row.rem_euclid(rows) as usize;
            let grid_col = new_col.rem_euclid(cols) as usize;
            
            // Check if it's a valid garden plot
            if grid[grid_row][grid_col] == '#' {
                continue;
            }
            
            let new_state = (new_row, new_col, step + 1);
            
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back(new_state);
            }
        }
    }
    
    reachable_at_target.len()
}

/// Part 2: Quadratic extrapolation for infinite grid
///
/// **The Challenge**: Cannot brute-force 26,501,365 steps!
///
/// **Key Insight**: Step count is NOT arbitrary:
/// ```
/// 26,501,365 = 65 + (131 × 202,300)
///              ↑     ↑      ↑
///         edge_dist  grid   periods
///                    size
/// ```
///
/// **Why Quadratic?** On infinite 2D grid, reachable area forms diamond:
/// - Area ≈ 2r² (Manhattan distance)
/// - General form: f(n) = an² + bn + c
///
/// **Algorithm**:
/// 1. Sample 3 points (minimum for quadratic):
///    - f(0) at 65 steps (reach edge, n=0)
///    - f(1) at 196 steps (65 + 131, n=1)
///    - f(2) at 327 steps (65 + 262, n=2)
///
/// 2. Fit quadratic using Lagrange interpolation:
///    ```
///    a = (n0 - 2n1 + n2) / 2
///    b = (-3n0 + 4n1 - n2) / 2
///    c = n0
///    ```
///
/// 3. Extrapolate to target:
///    ```
///    n = (26,501,365 - 65) / 131 = 202,300
///    f(202,300) = a×n² + b×n + c
///    ```
///
/// **Mathematical Foundation**:
/// - Lagrange interpolation: [[math-foundations/polynomial-interpolation-lagrange]]
/// - Finite differences validate quadratic: Δ²₀ = 2a
///
/// **Performance**: 3 BFS runs (~1.89s) vs 26M iterations (days/weeks)
/// - **Speedup**: ~800,000× faster!
///
/// **Type Safety**: Uses `i64` for coefficients (can be negative during calculation),
/// casts to `usize` for final result (counts can't be negative).
///
/// **See**: `day21_function_guide.md` for complete mathematical analysis
pub fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let grid_size = grid.len(); // Assuming square grid
    let start_infinite = (start.0 as isize, start.1 as isize);
    
    // Key insight: 26501365 = 65 + 131*202300
    // Where 65 is distance to edge, 131 is grid size
    // This means we can find a quadratic pattern!
    
    // Calculate for 3 data points: 65, 65+131, 65+262
    let edge_dist = grid_size / 2; // 65 for 131x131 grid
    
    let n0 = count_reachable_infinite(&grid, start_infinite, edge_dist);
    let n1 = count_reachable_infinite(&grid, start_infinite, edge_dist + grid_size);
    let n2 = count_reachable_infinite(&grid, start_infinite, edge_dist + 2 * grid_size);
    
    // Fit quadratic: f(x) = ax² + bx + c
    // We have f(0)=n0, f(1)=n1, f(2)=n2
    // Using Lagrange interpolation / finite differences
    
    let n0 = n0 as i64;
    let n1 = n1 as i64;
    let n2 = n2 as i64;
    
    let a = (n0 - 2 * n1 + n2) / 2;
    let b = (-3 * n0 + 4 * n1 - n2) / 2;
    let c = n0;
    
    // Target: (26501365 - 65) / 131 = 202300
    let target_n = ((26501365 - edge_dist) / grid_size) as i64;
    
    // Evaluate quadratic at target_n
    (a * target_n * target_n + b * target_n + c) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_example_6_steps() {
        let (grid, start) = parse_input(EXAMPLE);
        assert_eq!(count_reachable(&grid, start, 6), 16);
    }
    
    #[test]
    fn test_infinite_grid_example() {
        let (grid, start) = parse_input(EXAMPLE);
        let start_inf = (start.0 as isize, start.1 as isize);
        
        // Test example data points
        assert_eq!(count_reachable_infinite(&grid, start_inf, 6), 16);
        assert_eq!(count_reachable_infinite(&grid, start_inf, 10), 50);
        assert_eq!(count_reachable_infinite(&grid, start_inf, 50), 1594);
        assert_eq!(count_reachable_infinite(&grid, start_inf, 100), 6536);
    }
}
