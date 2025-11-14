use anyhow::{Context, Result};
use mission6::{Coord, Grid};
use std::collections::HashMap;
use std::collections::HashSet;

type AntennaMap = HashMap<char, Vec<Coord>>;
type ParsedInput = (Grid<char>, AntennaMap);

/// Parse antenna map into a Mission6 `Grid<char>` and frequency -> coordinates mapping.
fn parse_input(input: &str) -> Result<ParsedInput> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        anyhow::bail!("Input empty");
    }
    let height = lines.len();
    let width = lines[0].len();
    for (i, line) in lines.iter().enumerate() {
        if line.len() != width {
            anyhow::bail!("Non-rectangular grid at line {}: got {}, expected {}", i, line.len(), width);
        }
    }
    let mut grid = Grid::new(width, height, '.');
    let mut freq_map: HashMap<char, Vec<Coord>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coord = Coord::new(x, y);
            grid[coord] = ch;
            if ch != '.' { // antenna present
                freq_map.entry(ch).or_default().push(coord);
            }
        }
    }
    Ok((grid, freq_map))
}

/// Part 1 antinode calculation.
/// For each pair of same-frequency antennas p1,p2 produce two antinodes at p1 - (p2-p1) and p2 + (p2-p1) if in bounds.
pub fn solve_part1(input: &str) -> Result<String> {
    let (grid, freq_map) = parse_input(input).context("Failed to parse input for part1")?;
    let mut antinodes: HashSet<Coord> = HashSet::new();
    let width = grid.width();
    let height = grid.height();

    for (_freq, positions) in freq_map.iter() {
        if positions.len() < 2 { continue; }
        for i in 0..positions.len() {
            for j in (i+1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let dx = p2.x as isize - p1.x as isize;
                let dy = p2.y as isize - p1.y as isize;
                // Antinode before p1
                let a1x = p1.x as isize - dx;
                let a1y = p1.y as isize - dy;
                if a1x >= 0 && a1x < width as isize && a1y >= 0 && a1y < height as isize {
                    antinodes.insert(Coord::new(a1x as usize, a1y as usize));
                }
                // Antinode after p2
                let a2x = p2.x as isize + dx;
                let a2y = p2.y as isize + dy;
                if a2x >= 0 && a2x < width as isize && a2y >= 0 && a2y < height as isize {
                    antinodes.insert(Coord::new(a2x as usize, a2y as usize));
                }
            }
        }
    }
    Ok(antinodes.len().to_string())
}

/// Greatest common divisor (Euclidean algorithm) for non-negative isize values.
fn gcd(mut a: isize, mut b: isize) -> isize {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let r = a % b; a = b; b = r; }
    a
}

/// Debug helper: for each frequency with at least two antennas, print all pairs with
/// their raw (dx, dy) and gcd-normalized step (dx/g, dy/g).
///
/// Intended for manual inspection when exploring the geometry of the puzzle.
pub fn debug_print_pairs_with_gcd(input: &str) -> Result<()> {
    let (_grid, freq_map) = parse_input(input).context("Failed to parse input for debug_print_pairs_with_gcd")?;

    for (freq, positions) in freq_map.iter() {
        if positions.len() < 2 {
            continue;
        }
        println!("Frequency {freq}: {} antennas", positions.len());
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let dx = p2.x as isize - p1.x as isize;
                let dy = p2.y as isize - p1.y as isize;
                let g = gcd(dx, dy);
                let step_x = dx / g;
                let step_y = dy / g;
                println!(
                    "  pair ({}, {}) <-> ({}, {}): dx={}, dy={}, gcd={}, step=({}, {})",
                    p1.x, p1.y, p2.x, p2.y, dx, dy, g, step_x, step_y
                );
            }
        }
    }

    Ok(())
}

/// Part 2: Harmonic resonance - all positions along the line defined by any pair of same-frequency antennas.
pub fn solve_part2(input: &str) -> Result<String> {
    let (grid, freq_map) = parse_input(input).context("Failed to parse input for part2")?;
    let mut antinodes: HashSet<Coord> = HashSet::new();
    let width = grid.width() as isize;
    let height = grid.height() as isize;

    for (_freq, positions) in freq_map.iter() {
        if positions.len() < 2 { continue; }
        // Ensure original antenna positions count as antinodes under harmonic rule
        for &p in positions { antinodes.insert(p); }
        for i in 0..positions.len() {
            for j in (i+1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let dx = p2.x as isize - p1.x as isize;
                let dy = p2.y as isize - p1.y as isize;
                let g = gcd(dx, dy);
                let step_x = dx / g;
                let step_y = dy / g;
                // Radiate forward from p1
                let mut x = p1.x as isize + step_x;
                let mut y = p1.y as isize + step_y;
                while x >= 0 && x < width && y >= 0 && y < height {
                    antinodes.insert(Coord::new(x as usize, y as usize));
                    x += step_x; y += step_y;
                }
                // Radiate backward from p1
                let mut x2 = p1.x as isize - step_x;
                let mut y2 = p1.y as isize - step_y;
                while x2 >= 0 && x2 < width && y2 >= 0 && y2 < height {
                    antinodes.insert(Coord::new(x2 as usize, y2 as usize));
                    x2 -= step_x; y2 -= step_y;
                }
            }
        }
    }
    Ok(antinodes.len().to_string())
}

/// Variant of part2 that uses the full pair delta (dx, dy) as the step
/// instead of the gcd-normalized primitive step. This matches many
/// published AoC solutions and is useful for comparison.
pub fn solve_part2_raw_step(input: &str) -> Result<String> {
    let (grid, freq_map) = parse_input(input).context("Failed to parse input for part2_raw_step")?;
    let mut antinodes: HashSet<Coord> = HashSet::new();
    let width = grid.width() as isize;
    let height = grid.height() as isize;

    for (_freq, positions) in freq_map.iter() {
        if positions.len() < 2 { continue; }
        // Original antenna positions count as antinodes under harmonic rule
        for &p in positions { antinodes.insert(p); }
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let dx = p2.x as isize - p1.x as isize;
                let dy = p2.y as isize - p1.y as isize;

                // Step forward from p2
                let mut x = p2.x as isize + dx;
                let mut y = p2.y as isize + dy;
                while x >= 0 && x < width && y >= 0 && y < height {
                    antinodes.insert(Coord::new(x as usize, y as usize));
                    x += dx; y += dy;
                }

                // Step backward from p1
                let mut x2 = p1.x as isize - dx;
                let mut y2 = p1.y as isize - dy;
                while x2 >= 0 && x2 < width && y2 >= 0 && y2 < height {
                    antinodes.insert(Coord::new(x2 as usize, y2 as usize));
                    x2 -= dx; y2 -= dy;
                }
            }
        }
    }
    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example from problem statement (first diagram)
    const EXAMPLE_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_parse_rectangular() {
        let (grid, map) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(grid.width(), 12);
        assert_eq!(grid.height(), 12);
        assert!(map.get(&'0').unwrap().len() >= 4);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "14");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "34");
    }

    #[test]
    fn test_single_antenna_no_antinode_part1() {
        let input = "A"; // Single antenna only
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_two_antenna_part1() {
        let input = "A.A"; // width 3
        let result = solve_part1(input).unwrap();
        // Antennae at (0,0) and (2,0) -> dx=2 so antinodes at (-2,0) (out), (4,0) (out) => 0
        assert_eq!(result, "0");
    }

    #[test]
    fn test_harmonic_line_generation() {
        // Three aligned antennas horizontally should fill entire row under part2
        let input = "A.A.A"; // positions 0,2,4
        let result = solve_part2(input).unwrap();
        // All positions along line become antinodes: indices 0..=4 => 5
        assert_eq!(result, "5");
    }

    // #[test]
    // fn compare_gcd_vs_raw_on_greater_than_one_gcd() {
    //     // NOTE: This test is intentionally commented out because it
    //     // demonstrates a *difference* between the gcd-based implementation
    //     // (`solve_part2`) and the raw-step variant (`solve_part2_raw_step`).
    //     //
    //     // Custom grid: 7x4
    //     // A . . . . . .
    //     // . . . . . . .
    //     // . . . . . . .
    //     // . . . . . . A
    //     // Antennas at (0,0) and (6,3) with dx=6, dy=3, gcd=3
    //     let input = "A......\n.......\n.......\n......A\n";
    //
    //     let gcd_based = solve_part2(input).unwrap();
    //     let raw_based = solve_part2_raw_step(input).unwrap();
    //
    //     // Here `gcd_based` returns "4" (it visits all lattice points
    //     // along the line using primitive step (2,1): (0,0),(2,1),(4,2),(6,3)),
    //     // while `raw_based` returns "2" (it only sees the endpoints when
    //     // stepping by ±(6,3)). Enabling this assertion would therefore
    //     // deliberately fail, documenting why the gcd-based approach is
    //     // strictly more general than the raw-step one.
    //     // assert_eq!(gcd_based, raw_based);
    // }

    #[test]
    fn debug_example_pairs_with_gcd() {
        let input = include_str!("../../inputs/day08_example.txt");
        debug_print_pairs_with_gcd(input).unwrap();
    }
}