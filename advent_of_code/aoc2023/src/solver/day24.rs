//! # Day 24: Never Tell Me The Odds
//!
//! ## Part 1 - 2D Line Intersection
//!
//! Given hailstones with 3D positions and velocities, find how many pairs of paths
//! will intersect in the future within a test area (ignoring Z axis).
//!
//! ## Algorithm
//! - Parse: Extract position (px, py, pz) and velocity (vx, vy, vz)
//! - Parametric lines: Each hailstone follows (x, y) = (px, py) + t * (vx, vy)
//! - Intersection: Solve system for t₁, t₂ where paths meet
//! - Validation: Both t₁, t₂ > 0 (future), point within bounds
//!
//! ## Mathematics
//! Line intersection using parametric equations:
//! - Stone A: x = px_a + t_a * vx_a, y = py_a + t_a * vy_a
//! - Stone B: x = px_b + t_b * vx_b, y = py_b + t_b * vy_b
//! - At intersection: px_a + t_a * vx_a = px_b + t_b * vx_b (same for y)
//! - Solve 2×2 system for t_a, t_b
//! - Parallel check: determinant = 0 (vx_a * vy_b - vy_a * vx_b = 0)
//!
//! ## Complexity
//! - Parse: O(n)
//! - Check all pairs: O(n²)
//! - Per-pair: O(1) line intersection
//! - Total: O(n²)

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: f64,
    py: f64,
    #[allow(dead_code)] // Used in Part 2
    pz: f64,
    vx: f64,
    vy: f64,
    #[allow(dead_code)] // Used in Part 2
    vz: f64,
}

impl Hailstone {
    /// Parse from format: "px, py, pz @ vx, vy, vz"
    fn parse(line: &str) -> Result<Self> {
        let (pos, vel) = line
            .split_once('@')
            .context("Invalid format: missing '@'")?;

        let pos_nums: Vec<f64> = pos
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect::<Result<_, _>>()
            .context("Failed to parse position")?;

        let vel_nums: Vec<f64> = vel
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect::<Result<_, _>>()
            .context("Failed to parse velocity")?;

        if pos_nums.len() != 3 || vel_nums.len() != 3 {
            anyhow::bail!("Expected 3 position and 3 velocity values");
        }

        Ok(Hailstone {
            px: pos_nums[0],
            py: pos_nums[1],
            pz: pos_nums[2],
            vx: vel_nums[0],
            vy: vel_nums[1],
            vz: vel_nums[2],
        })
    }

    /// Check if two hailstones' paths will intersect in the future within bounds.
    ///
    /// Returns true if:
    /// - Paths intersect (not parallel)
    /// - Intersection happens in future for both (t > 0)
    /// - Intersection point within test area bounds
    fn intersects_2d_in_bounds(&self, other: &Hailstone, min: f64, max: f64) -> bool {
        // Parametric line equations (ignoring Z):
        // Stone A (self): (x, y) = (px_a, py_a) + t_a * (vx_a, vy_a)
        // Stone B (other): (x, y) = (px_b, py_b) + t_b * (vx_b, vy_b)
        //
        // At intersection, both equations must equal:
        // px_a + t_a * vx_a = px_b + t_b * vx_b  ... (1)
        // py_a + t_a * vy_a = py_b + t_b * vy_b  ... (2)
        //
        // Rearranging:
        // t_a * vx_a - t_b * vx_b = px_b - px_a  ... (1')
        // t_a * vy_a - t_b * vy_b = py_b - py_a  ... (2')
        //
        // Using elimination or Cramer's rule:
        // From (1'): t_a = (px_b - px_a + t_b * vx_b) / vx_a
        // Substitute into (2'):
        // ((px_b - px_a + t_b * vx_b) / vx_a) * vy_a - t_b * vy_b = py_b - py_a
        // (px_b - px_a) * vy_a + t_b * vx_b * vy_a - t_b * vy_b * vx_a = (py_b - py_a) * vx_a
        // t_b * (vx_b * vy_a - vy_b * vx_a) = (py_b - py_a) * vx_a - (px_b - px_a) * vy_a
        //
        // So: t_b = [(py_b - py_a) * vx_a - (px_b - px_a) * vy_a] / (vx_b * vy_a - vy_b * vx_a)
        // And: t_a = [(py_b - py_a) * vx_b - (px_b - px_a) * vy_b] / (vx_b * vy_a - vy_b * vx_a)
        //
        // Let det = vx_b * vy_a - vy_b * vx_a (cross product)
        // If det = 0, lines are parallel

        let dx = other.px - self.px;
        let dy = other.py - self.py;

        // Determinant (cross product: other.v × self.v)
        let det = other.vx * self.vy - other.vy * self.vx;

        // Parallel lines (determinant ~ 0)
        if det.abs() < 1e-10 {
            return false;
        }

        // Solve for intersection times
        let t_a = (dy * other.vx - dx * other.vy) / det;
        let t_b = (dy * self.vx - dx * self.vy) / det;

        // Check if intersection is in the future for both hailstones
        if t_a < 0.0 || t_b < 0.0 {
            return false;
        }

        // Calculate intersection point (using t_a with self's equation)
        let ix = self.px + t_a * self.vx;
        let iy = self.py + t_a * self.vy;

        // Check if intersection is within bounds
        ix >= min && ix <= max && iy >= min && iy <= max
    }
}

/// **Day 24: Never Tell Me The Odds - Part 1**
///
/// Count pairs of hailstone paths that intersect in the future within test area.
pub fn solve_part1(input: &str) -> Result<String> {
    solve_part1_with_bounds(input, 200_000_000_000_000.0, 400_000_000_000_000.0)
}

fn solve_part1_with_bounds(input: &str, min: f64, max: f64) -> Result<String> {
    // Parse all hailstones
    let hailstones: Vec<Hailstone> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Hailstone::parse)
        .collect::<Result<_>>()?;

    // Check all pairs for intersections
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            if hailstones[i].intersects_2d_in_bounds(&hailstones[j], min, max) {
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}

pub fn solve_part2(_input: &str) -> Result<String> {
    solve_part2_linear_system(_input)
    // ILP approach commented out - requires more complex constraint formulation
    // match solve_part2_linear_system(_input) {
    //     Ok(result) => Ok(result),
    //     Err(_) => solve_part2_ilp(_input),
    // }
}

/// **Part 2: Solve using Integer Linear Programming**
///
/// **Part 2: Solve using linear system of equations**
///
/// Key insight: For two hailstones i and j both hit by the rock:
/// - At time ti: rx + ti*rvx = pxi + ti*vxi
/// - At time tj: rx + tj*rvx = pxj + tj*vxj
///
/// Subtracting to eliminate rx:
/// (tj - ti)*rvx = (pxj - pxi) + tj*vxj - ti*vxi
///
/// This is still nonlinear, but we can use cross products to eliminate time.
/// For hailstones i and j, the relative position and velocity vectors
/// must satisfy certain constraints.
///
/// **Algorithm**:
/// 1. Build 6 linear equations from 4 hailstones using cross-product elimination
/// 2. Solve using Gaussian elimination to get floating-point solution
/// 3. The solution has rounding errors, so we search a 3x3x3 neighborhood
///    around floor(solution) to find integer coordinates minimizing collision error
///
/// **Answer**: 546494494317645 (sum of rock's initial position coordinates)
fn solve_part2_linear_system(input: &str) -> Result<String> {
    // Parse all hailstones
    let hailstones: Vec<Hailstone> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Hailstone::parse)
        .collect::<Result<_>>()?;

    anyhow::ensure!(hailstones.len() >= 4, "Need at least 4 hailstones");

    // Use multiple hailstone combinations and try to find exact integer solution
    // We'll try different sets of 4 hailstones to see if we get consistent results

    // The key equation: For stones A and B both hit by rock R:
    // (pB - pA) × (vB - vA) = (pB - pA) × (vR - vA) + (vB - vA) × (pR - pA)
    //
    // Expanding and collecting terms gives us linear equations in (pR, vR)

    // Try using hailstones 0,1,2,3 first, then try others if needed
    let indices = [0, 1, 2, 3];
    let h0 = &hailstones[indices[0]];
    let h1 = &hailstones[indices[1]];
    let h2 = &hailstones[indices[2]];
    let h3 = &hailstones[indices[3]];

    // Build linear system Ax = b where x = [rx, ry, rz, rvx, rvy, rvz]
    // We'll use Gaussian elimination to solve it

    let mut matrix: Vec<Vec<f64>> = vec![];
    let mut constants: Vec<f64> = vec![];

    // Add equations from pairs (h0, h1), (h0, h2), (h0, h3)
    for h in [h1, h2, h3] {
        // From cross product elimination, we get 2 equations per pair
        // These come from setting equal the cross products of position/velocity

        // Equation 1: From Y-Z components
        let a1 = h.vy - h0.vy;
        let a2 = h0.vx - h.vx;
        let a3 = 0.0;
        let a4 = h0.py - h.py;
        let a5 = h.px - h0.px;
        let a6 = 0.0;
        let b1 = h0.py * h0.vx - h0.px * h0.vy - h.py * h.vx + h.px * h.vy;

        matrix.push(vec![a1, a2, a3, a4, a5, a6]);
        constants.push(b1);

        // Equation 2: From X-Z components
        let a1 = h.vz - h0.vz;
        let a2 = 0.0;
        let a3 = h0.vx - h.vx;
        let a4 = h0.pz - h.pz;
        let a5 = 0.0;
        let a6 = h.px - h0.px;
        let b2 = h0.pz * h0.vx - h0.px * h0.vz - h.pz * h.vx + h.px * h.vz;

        matrix.push(vec![a1, a2, a3, a4, a5, a6]);
        constants.push(b2);
    }

    // Solve using Gaussian elimination
    let solution = gaussian_elimination(&mut matrix, &mut constants)?;

    let rx = solution[0];
    let ry = solution[1];
    let rz = solution[2];
    let rvx = solution[3];
    let rvy = solution[4];
    let rvz = solution[5];

    // The solution from Gaussian elimination might have floating-point errors
    // Try rounding each component and check which gives the best fit

    // The Gaussian elimination solution has floating-point rounding errors
    // Use floor as starting point for local search
    let rx_int = rx.floor() as i64;
    let ry_int = ry.floor() as i64;
    let rz_int = rz.floor() as i64;
    // Search a 3x3x3 neighborhood around the floor solution to find
    // the integer coordinates that minimize collision error

    let mut best_answer = rx_int + ry_int + rz_int;
    let mut best_error = f64::MAX;

    // Search small neighborhood around the floating-point solution
    for dx in [-1i64, 0, 1] {
        for dy in [-1, 0, 1] {
            for dz in [-1, 0, 1] {
                let test_rx = rx_int + dx;
                let test_ry = ry_int + dy;
                let test_rz = rz_int + dz;

                // Check collision error with first 3 hailstones
                let mut total_error = 0.0;
                let mut valid = true;

                for h in [h0, h1, h2] {
                    if (h.vx - rvx).abs() > 1e-10 {
                        let t = (test_rx as f64 - h.px) / (h.vx - rvx);
                        if t > 0.0 {
                            let stone_x = h.px + t * h.vx;
                            let stone_y = h.py + t * h.vy;
                            let stone_z = h.pz + t * h.vz;
                            let rock_x = test_rx as f64 + t * rvx;
                            let rock_y = test_ry as f64 + t * rvy;
                            let rock_z = test_rz as f64 + t * rvz;

                            let err = (stone_x - rock_x).abs()
                                + (stone_y - rock_y).abs()
                                + (stone_z - rock_z).abs();
                            total_error += err;
                        } else {
                            valid = false;
                            break;
                        }
                    }
                }

                if valid && total_error < best_error {
                    best_error = total_error;
                    best_answer = test_rx + test_ry + test_rz;
                }
            }
        }
    }

    Ok(best_answer.to_string())
}

/// Solve linear system Ax = b using Gaussian elimination with partial pivoting
fn gaussian_elimination(a: &mut [Vec<f64>], b: &mut [f64]) -> Result<Vec<f64>> {
    let n = b.len();
    anyhow::ensure!(a.len() == n, "Matrix dimensions mismatch");
    anyhow::ensure!(a[0].len() >= n, "Need at least as many columns as rows");

    // Forward elimination with partial pivoting
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }

        // Swap rows
        a.swap(i, max_row);
        b.swap(i, max_row);

        // Check for singular matrix
        if a[i][i].abs() < 1e-10 {
            anyhow::bail!("Singular matrix");
        }

        // Eliminate column
        for k in (i + 1)..n {
            let factor = a[k][i] / a[i][i];
            b[k] -= factor * b[i];
            for j in i..a[0].len() {
                a[k][j] -= factor * a[i][j];
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_parse_hailstone() {
        let stone = Hailstone::parse("19, 13, 30 @ -2, 1, -2").unwrap();
        assert_eq!(stone.px, 19.0);
        assert_eq!(stone.py, 13.0);
        assert_eq!(stone.pz, 30.0);
        assert_eq!(stone.vx, -2.0);
        assert_eq!(stone.vy, 1.0);
        assert_eq!(stone.vz, -2.0);
    }

    #[test]
    fn test_part1_example() {
        // Example uses bounds [7, 27] instead of the large test area
        let result = solve_part1_with_bounds(EXAMPLE, 7.0, 27.0).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        // Example solution: position (24, 13, 10) => 24 + 13 + 10 = 47
        assert_eq!(result, "47");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day24.txt");
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "546494494317645");
    }

    #[test]
    fn test_intersection_manual() {
        // Stone A: position (19, 13), velocity (-2, 1)
        // Stone B: position (18, 19), velocity (-1, -1)
        // Expected intersection: (14.333, 15.333) at t_a=2.333, t_b=3.667

        let stone_a = Hailstone::parse("19, 13, 30 @ -2, 1, -2").unwrap();
        let stone_b = Hailstone::parse("18, 19, 22 @ -1, -1, -2").unwrap();

        // Verify expected intersection point works for both stones
        let t_a = 2.333333;
        let t_b = 3.666667;

        let a_x = stone_a.px + t_a * stone_a.vx;
        let a_y = stone_a.py + t_a * stone_a.vy;
        let b_x = stone_b.px + t_b * stone_b.vx;
        let b_y = stone_b.py + t_b * stone_b.vy;

        assert!((a_x - 14.333).abs() < 0.01);
        assert!((a_y - 15.333).abs() < 0.01);
        assert!((b_x - 14.333).abs() < 0.01);
        assert!((b_y - 15.333).abs() < 0.01);

        // Now test our function
        assert!(stone_a.intersects_2d_in_bounds(&stone_b, 7.0, 27.0));
    }

    #[test]
    fn test_intersection_cases() {
        let stone_a = Hailstone::parse("19, 13, 30 @ -2, 1, -2").unwrap();
        let stone_b = Hailstone::parse("18, 19, 22 @ -1, -1, -2").unwrap();

        // Should intersect inside test area at (14.333, 15.333)
        assert!(stone_a.intersects_2d_in_bounds(&stone_b, 7.0, 27.0));
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day24.txt");
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "18651"); // Verified correct answer
    }

    #[test]
    fn test_parallel_lines() {
        let stone_a = Hailstone::parse("18, 19, 22 @ -1, -1, -2").unwrap();
        let stone_b = Hailstone::parse("20, 25, 34 @ -2, -2, -4").unwrap();

        // Parallel lines (velocities are proportional: -1:-1 and -2:-2)
        assert!(!stone_a.intersects_2d_in_bounds(&stone_b, 7.0, 27.0));
    }

    #[test]
    fn test_past_intersection() {
        let stone_a = Hailstone::parse("19, 13, 30 @ -2, 1, -2").unwrap();
        let stone_b = Hailstone::parse("20, 19, 15 @ 1, -5, -3").unwrap();

        // Paths crossed in the past for stone A
        assert!(!stone_a.intersects_2d_in_bounds(&stone_b, 7.0, 27.0));
    }
}
