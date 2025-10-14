use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Day 2: I Was Told There Would Be No Math (Alternative Implementation)
// This version uses read_lines for direct file reading instead of the standard AoC infrastructure

/// Read lines from a file and return as Vec<String>
pub fn read_lines<P: AsRef<Path>>(p: P) -> Result<Vec<String>> {
    let f = fs::File::open(p)?;
    let r = BufReader::new(f);
    Ok(r.lines().collect::<Result<_, _>>()?)
}

/// Parse a line like "14x12x8" into (14, 12, 8)
fn parse_dimensions(line: &str) -> Result<(i32, i32, i32)> {
    let parts: Vec<&str> = line.split('x').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!(
            "Invalid format: expected 3 dimensions separated by 'x'"
        ));
    }

    let length = parts[0].parse::<i32>()?;
    let width = parts[1].parse::<i32>()?;
    let height = parts[2].parse::<i32>()?;

    Ok((length, width, height))
}

/// Solve part 1 by reading directly from file
pub fn solve_part1_from_file<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let mut total_paper = 0;

    // Read all lines from file
    let lines = read_lines(file_path)?;

    // Process each line
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (l, w, h) = parse_dimensions(&line)?;

        // Calculate surface area: 2*l*w + 2*w*h + 2*h*l
        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;

        // Find smallest side area for extra paper
        let smallest_side = (l * w).min(w * h).min(h * l);

        total_paper += surface_area + smallest_side;
    }

    Ok(total_paper.to_string())
}

/// Solve part 2 by reading directly from file
pub fn solve_part2_from_file<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let mut total_ribbon = 0;

    // Read all lines from file
    let lines = read_lines(file_path)?;

    // Process each line
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (l, w, h) = parse_dimensions(&line)?;

        // Calculate ribbon needed: smallest perimeter + bow (volume)
        let mut sides = vec![l, w, h];
        sides.sort();
        let smallest_perimeter = 2 * sides[0] + 2 * sides[1];
        let bow = l * w * h;

        total_ribbon += smallest_perimeter + bow;
    }

    Ok(total_ribbon.to_string())
}

/// Standard solve_part1 for compatibility with existing infrastructure
pub fn solve_part1(input: &str) -> Result<String> {
    let mut total_paper = 0;

    // Cycle through all lines in the input
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let (l, w, h) = parse_dimensions(line)?;

        // Calculate surface area: 2*l*w + 2*w*h + 2*h*l
        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;

        // Find smallest side area for extra paper
        let smallest_side = (l * w).min(w * h).min(h * l);

        total_paper += surface_area + smallest_side;
    }

    Ok(total_paper.to_string())
}

/// Standard solve_part2 for compatibility with existing infrastructure
pub fn solve_part2(input: &str) -> Result<String> {
    let mut total_ribbon = 0;

    // Cycle through all lines in the input
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let (l, w, h) = parse_dimensions(line)?;

        // Calculate ribbon needed: smallest perimeter + bow (volume)
        let mut sides = vec![l, w, h];
        sides.sort();
        let smallest_perimeter = 2 * sides[0] + 2 * sides[1];
        let bow = l * w * h;

        total_ribbon += smallest_perimeter + bow;
    }

    Ok(total_ribbon.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dimensions() {
        let result = parse_dimensions("2x3x4").unwrap();
        assert_eq!(result, (2, 3, 4));

        let result = parse_dimensions("14x12x8").unwrap();
        assert_eq!(result, (14, 12, 8));
    }

    #[test]
    fn test_parse_dimensions_invalid() {
        assert!(parse_dimensions("2x3").is_err());
        assert!(parse_dimensions("2x3x4x5").is_err());
        assert!(parse_dimensions("2xABCx4").is_err());
    }

    #[test]
    fn test_wrapping_paper_calculation() {
        // Example from problem: 2x3x4 should need 58 square feet
        // Surface area: 2*2*3 + 2*3*4 + 2*4*2 = 12 + 24 + 16 = 52
        // Smallest side: min(2*3, 3*4, 4*2) = min(6, 12, 8) = 6
        // Total: 52 + 6 = 58

        let input = "2x3x4";
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "58");
    }

    #[test]
    fn test_ribbon_calculation() {
        // Example from problem: 2x3x4 should need 34 feet of ribbon
        // Smallest perimeter: 2*2 + 2*3 = 10 (around 2x3 face)
        // Bow: 2*3*4 = 24
        // Total: 10 + 24 = 34

        let input = "2x3x4";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "34");
    }

    #[test]
    fn test_multiple_lines() {
        let input = "2x3x4\n1x1x10";

        // First box: 58 (as calculated above)
        // Second box: 1x1x10
        // Surface: 2*1*1 + 2*1*10 + 2*10*1 = 2 + 20 + 20 = 42
        // Smallest: min(1*1, 1*10, 10*1) = min(1, 10, 10) = 1
        // Total for second: 42 + 1 = 43
        // Combined: 58 + 43 = 101

        let result = solve_part1(input).unwrap();
        assert_eq!(result, "101");
    }
}

fn main() {
    println!("Alternative Day 2 implementation examples");
    println!("This file contains alternative implementations and tests.");
}
